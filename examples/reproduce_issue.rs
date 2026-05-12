use encfs_cracker::crypto::encfs_config::EncfSConfig;

fn main() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE boost_serialization>
<boost_serialization signature="serialization::archive" version="7">
    <cfg class_id="0" tracking_level="0" version="20">
        <version>20100713</version>
        <creator>EncFS 1.9.5</creator>
        <cipherAlg class_id="1" tracking_level="0" version="0">
            <name>ssl/aes</name>
            <major>3</major>
            <minor>0</minor>
        </cipherAlg>
        <nameAlg>
            <name>nameio/block</name>
            <major>4</major>
            <minor>0</minor>
        </nameAlg>
        <keySize>256</keySize>
        <blockSize>1024</blockSize>
        <plainData>0</plainData>
        <uniqueIV>1</uniqueIV>
        <chainedNameIV>1</chainedNameIV>
        <externalIVChaining>1</externalIVChaining>
        <blockMACBytes>8</blockMACBytes>
        <blockMACRandBytes>0</blockMACRandBytes>
        <allowHoles>1</allowHoles>
        <encodedKeySize>52</encodedKeySize>
        <encodedKeyData>
elnyw1B96uMzdGh5CKTXG44EZ25ztz3CvP7tNtsaxbjvqZHwWErHAcZ8SBTiLz4RswtCSw==
</encodedKeyData>
        <saltLen>20</saltLen>
        <saltData>
8uwRcAXFa76Dvu5mXdNEFUBS4cQ=
</saltData>
        <kdfIterations>3259949</kdfIterations>
        <desiredKDFDuration>3000</desiredKDFDuration>
    </cfg>
</boost_serialization>"#;

    let config = EncfSConfig::from_xml(xml).expect("Failed to parse XML");
    
    let passwords = vec!["123456", "wrongpassword"];
    
    for password in passwords {
        println!("Testing password: {}", password);
        if config.verify_password(password) {
            println!("RESULT: Password '{}' verified!", password);
        } else {
            println!("RESULT: Password '{}' NOT verified.", password);
        }
    }
}
