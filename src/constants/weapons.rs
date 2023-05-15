use mongodb::bson::oid::ObjectId;
use crate::models::Weapon;

pub fn weapon_d2() -> Vec<Weapon> {
    vec![
        Weapon {
            id: ObjectId::new(),
            name: String::from("玉兔"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("救赎之握"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("北欧化工"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("真相"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("冷漠的心"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("探矿者"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("Crosethia 77k"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("两尾狐"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("利维坦之息"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("普罗米修斯镜头"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("达西"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("引力子矛"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("甜蜜的生意"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("世界零"),
            power_level: 100,
        },
        Weapon {
            id: ObjectId::new(),
            name: String::from("地狱犬+1"),
            power_level: 100,
        },
    ]   
}