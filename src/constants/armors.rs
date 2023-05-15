use mongodb::bson::oid::ObjectId;
use crate::models::Armor;

pub fn armor_d2() -> Vec<Armor> {
    vec![
        Armor {
            id: ObjectId::new(),
            name: String::from("炎阳护腕"),
            power_level: 100,
        },
        Armor {
            id: ObjectId::new(),
            name: String::from("卡恩斯坦臂章"),
            power_level: 100,
        },
        Armor {
            id: ObjectId::new(),
            name: String::from("狡诈严冬"),
            power_level: 100,
        },
        Armor {
            id: ObjectId::new(),
            name: String::from("星火协议"),
            power_level: 100,
        },
        Armor {
            id: ObjectId::new(),
            name: String::from("神圣黎明之翼"),
            power_level: 100,
        },
        //...rest of the weapons
    ]
}