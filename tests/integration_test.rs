/*
 * Copyright Stalwart Labs, Minter Ltd. See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use mail_parser::*;

#[test]
fn test_api() {
    let mut input = concat!(
        "From: Art Vandelay <art@vandelay.com> (Vandelay Industries)\n",
        "To: \"Colleagues\": \"James Smythe\" <james@vandelay.com>; Friends:\n",
        "    jane@example.com, =?UTF-8?Q?John_Sm=C3=AEth?= <john@example.com>;\n",
        "Date: Sat, 20 Nov 2021 14:22:01 -0800\n",
        "Subject: Why not both importing AND exporting? =?utf-8?b?4pi6?=\n",
        "Content-Type: multipart/mixed; boundary=\"festivus\";\n\n",
        "--festivus\n",
        "Content-Type: text/html; charset=\"us-ascii\"\n",
        "Content-Transfer-Encoding: base64\n\n",
        "PGh0bWw+PHA+SSB3YXMgdGhpbmtpbmcgYWJvdXQgcXVpdHRpbmcgdGhlICZsZHF1bztle\n",
        "HBvcnRpbmcmcmRxdW87IHRvIGZvY3VzIGp1c3Qgb24gdGhlICZsZHF1bztpbXBvcnRpbm\n",
        "cmcmRxdW87LDwvcD48cD5idXQgdGhlbiBJIHRob3VnaHQsIHdoeSBub3QgZG8gYm90aD8\n",
        "gJiN4MjYzQTs8L3A+PC9odG1sPg==\n",
        "--festivus\n",
        "Content-Type: message/rfc822\n\n",
        "From: \"Cosmo Kramer\" <kramer@kramerica.com>\n",
        "Subject: Exporting my book about coffee tables\n",
        "Content-Type: multipart/mixed; boundary=\"giddyup\";\n\n",
        "--giddyup\n",
        "Content-Type: text/plain; charset=\"utf-16\"\n",
        "Content-Transfer-Encoding: quoted-printable\n\n",
        "=FF=FE=0C!5=D8\"=DD5=D8)=DD5=D8-=DD =005=D8*=DD5=D8\"=DD =005=D8\"=\n",
        "=DD5=D85=DD5=D8-=DD5=D8,=DD5=D8/=DD5=D81=DD =005=D8*=DD5=D86=DD =\n",
        "=005=D8=1F=DD5=D8,=DD5=D8,=DD5=D8(=DD =005=D8-=DD5=D8)=DD5=D8\"=\n",
        "=DD5=D8=1E=DD5=D80=DD5=D8\"=DD!=00\n",
        "--giddyup\n",
        "Content-Type: image/gif; name*1=\"about \"; name*0=\"Book \";\n",
        "              name*2*=utf-8''%e2%98%95tables.gif\n",
        "Content-Transfer-Encoding: Base64\n",
        "Content-Disposition: attachment\n\n",
        "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7\n",
        "--giddyup--\n",
        "--festivus--\n",
    )
    .as_bytes()
    .to_vec();

    let message = Message::parse(&mut input[..]);

    assert_eq!(
        message.get_from(),
        &Address::Address(Addr {
            name: Some("Art Vandelay (Vandelay Industries)".into()),
            address: Some("art@vandelay.com".into())
        })
    );
    assert_eq!(
        message.get_to(),
        &Address::GroupList(vec![
            Group {
                name: Some("Colleagues".into()),
                addresses: vec![Addr {
                    name: Some("James Smythe".into()),
                    address: Some("james@vandelay.com".into())
                }]
            },
            Group {
                name: Some("Friends".into()),
                addresses: vec![
                    Addr {
                        name: None,
                        address: Some("jane@example.com".into())
                    },
                    Addr {
                        name: Some("John Smîth".into()),
                        address: Some("john@example.com".into())
                    }
                ]
            }
        ])
    );

    assert_eq!(
        message.get_date().unwrap().to_iso8601(),
        "2021-11-20T14:22:01-08:00"
    );
    assert_eq!(
        message.get_subject().unwrap(),
        "Why not both importing AND exporting? ☺"
    );
    assert_eq!(
        message.get_html_body(0).unwrap().to_string(),
        concat!(
            "<html><p>I was thinking about quitting the &ldquo;exporting&rdquo; to ",
            "focus just on the &ldquo;importing&rdquo;,</p><p>but then I thought,",
            " why not do both? &#x263A;</p></html>"
        )
    );
    assert_eq!(
        message.get_text_body(0).unwrap().to_string(),
        concat!(
            "I was thinking about quitting the “exporting” to focus just on the",
            " “importing”,\nbut then I thought, why not do both? ☺\n"
        )
    );
    let nested_message = match message.get_attachment(0).unwrap() {
        MessagePart::Message(v) => v,
        _ => unreachable!(),
    };
    assert_eq!(
        nested_message.get_subject().unwrap(),
        "Exporting my book about coffee tables"
    );
    assert_eq!(
        nested_message.get_text_body(0).unwrap().to_string(),
        "ℌ𝔢𝔩𝔭 𝔪𝔢 𝔢𝔵𝔭𝔬𝔯𝔱 𝔪𝔶 𝔟𝔬𝔬𝔨 𝔭𝔩𝔢𝔞𝔰𝔢!"
    );
    assert_eq!(
        nested_message.get_html_body(0).unwrap().to_string(),
        "<html><body>ℌ𝔢𝔩𝔭 𝔪𝔢 𝔢𝔵𝔭𝔬𝔯𝔱 𝔪𝔶 𝔟𝔬𝔬𝔨 𝔭𝔩𝔢𝔞𝔰𝔢!</body></html>"
    );
    let nested_attachment = match nested_message.get_attachment(0).unwrap() {
        MessagePart::Binary(v) => v,
        _ => unreachable!(),
    };
    assert_eq!(nested_attachment.len(), 42);
    assert_eq!(
        nested_attachment
            .get_header()
            .unwrap()
            .get_content_type()
            .unwrap()
            .get_attribute("name")
            .unwrap(),
        "Book about ☕tables.gif"
    );

    //println!("{}", serde_json::to_string_pretty(&message).unwrap());
    //println!("{}", serde_yaml::to_string(&message).unwrap());

}
