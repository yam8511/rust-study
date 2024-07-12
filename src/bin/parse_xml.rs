use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

fn main() {
    println!("Hello, world!");

    let xml = r#"
        <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <tt:MetadataStream xmlns:tt="http://www.onvif.org/ver10/schema"
            xmlns:wsnt="http://docs.oasis-open.org/wsn/b-2">
            <tt:VideoAnalytics>
                <tt:Frame UtcTime="2020-09-25T11:28:17.432Z">
                    <tt:Transformation>
                        <tt:Translate x="-1.0" y="1.0" />
                        <tt:Scale x="0.0002" y="-0.0002" />
                    </tt:Transformation>
                    <tt:Object ObjectId="100">
                        <tt:Appearance>
                            <tt:Shape>
                                <tt:BoundingBox left="375" top="3112" right="1875" bottom="7445" />
                                <tt:CenterOfGravity x="1125" y="5278" />
                            </tt:Shape>
                            <tt:LicensePlateInfo>
                                <tt:PlateNumber>ABC-1234</tt:PlateNumber>
                                <tt:Likelihood>0.9</tt:Likelihood>
                                <tt:PlateType>Normal</tt:PlateType>
                                <tt:CountryCode>US</tt:CountryCode>
                                <tt:IssuingEntity>California</tt:IssuingEntity>
                            </tt:LicensePlateInfo>
                            <tt:Class>
                                <tt:ClassCandidate>
                                    <tt:StreamSource>STREAM_2</tt:StreamSource>
                                </tt:ClassCandidate>
                            </tt:Class>
                        </tt:Appearance>
                        <tt:Behaviour />
                    </tt:Object>
                </tt:Frame>
            </tt:VideoAnalytics>
        </tt:MetadataStream>
    "#;
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut check_plate = false;
    let mut check_score = false;
    let mut temp_plate = None;
    let mut temp_score = None;
    let mut temp_center = None;
    let mut temp_bbox = None;

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => println!(
                "invalid xml, error at position {}: {:?}",
                reader.buffer_position(),
                e
            ),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"tt:BoundingBox" => {
                    let mut left = None;
                    let mut top = None;
                    let mut right = None;
                    let mut bottom = None;
                    e.attributes().for_each(|attr| {
                        if let Ok(attr) = attr {
                            println!("attr = {:?}", attr);
                            match attr.key {
                                QName(b"left") => left = Some(attr.value.to_owned().to_vec()),
                                QName(b"top") => top = Some(attr.value.to_owned().to_vec()),
                                QName(b"right") => right = Some(attr.value.to_owned().to_vec()),
                                QName(b"bottom") => bottom = Some(attr.value.to_owned().to_vec()),
                                _ => (),
                            }
                        }
                    });
                    if let (Some(left), Some(top), Some(right), Some(bottom)) =
                        (left, top, right, bottom)
                    {
                        if let (Ok(left), Ok(top), Ok(right), Ok(bottom)) = (
                            String::from_utf8(left),
                            String::from_utf8(top),
                            String::from_utf8(right),
                            String::from_utf8(bottom),
                        ) {
                            if let (Ok(left), Ok(top), Ok(right), Ok(bottom)) = (
                                left.parse::<i32>(),
                                top.parse::<i32>(),
                                right.parse::<i32>(),
                                bottom.parse::<i32>(),
                            ) {
                                temp_bbox = Some((left, top, right - left, bottom - top))
                            }
                        }
                    }
                }
                b"tt:CenterOfGravity" => {
                    let mut x = None;
                    let mut y = None;
                    e.attributes().for_each(|attr| {
                        if let Ok(attr) = attr {
                            println!("attr = {:?}", attr);
                            match attr.key {
                                QName(b"x") => x = Some(attr.value.to_owned().to_vec()),
                                QName(b"y") => y = Some(attr.value.to_owned().to_vec()),
                                _ => (),
                            }
                        }
                    });
                    if let (Some(x), Some(y)) = (x, y) {
                        if let (Ok(x), Ok(y)) = (String::from_utf8(x), String::from_utf8(y)) {
                            println!("x = {:?}", x);
                            println!("y = {:?}", y);
                            if let (Ok(x), Ok(y)) = (x.parse::<f32>(), y.parse::<f32>()) {
                                temp_center = Some((x, y))
                            }
                        }
                    }
                }
                v => println!("empty tag = {}", String::from_utf8(v.to_vec()).unwrap()),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"tt:PlateNumber" => check_plate = true,
                b"tt:Likelihood" => check_score = true,
                v => println!("tag = {}", String::from_utf8(v.to_vec()).unwrap()),
            },
            Ok(Event::Text(e)) => {
                if let Ok(text) = e.unescape() {
                    let text = text.into_owned();
                    println!("text => {}", text);
                    match (check_plate, check_score) {
                        (true, false) => temp_plate = Some(text),
                        (false, true) => {
                            if let Ok(score) = text.parse::<f32>() {
                                temp_score = Some(score)
                            }
                        }
                        _ => (),
                    };
                }
            }
            Ok(Event::End(_)) => {
                check_plate = false;
                check_score = false;
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    if let (Some(plate), Some(score), Some((x, y)), Some((x1, y1, w, h))) =
        (temp_plate, temp_score, temp_center, temp_bbox)
    {
        println!("Plate: {} {:.2} center = ({x:.2} {y:.2}), bbox = ({x1:.2}, {y1:.2}) width = {w:.2} height = {h:.2}", plate, score);
    }
}
