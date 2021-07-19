//Traditional Struct
/*struct AussieMusic {
    metal: u8,
    rap: u8,
    edm: u8,
    alternative: u8
    }
    //Tuple Struct
    struct Color(u8, u8, u8);
     */
    //According to pfaf.org
    struct Plant {
        name: String,
        origin: String,
        properties: String

    }
    impl Plant {
        fn new(title: &str, from: &str, prop: &str) -> Plant {
            Plant {
                name: title.to_string(),
                origin: from.to_string(),
                properties: prop.to_string()


            }
        }
    }
pub fn run() {
/*let mut a = AussieMusic {
    alternative: 1,
    edm: 2,
    rap: 3,
    metal: 4
};
a.rap = 2;
println!("Top Australian Music Genres: {} {} {} {}", a.alternative, a.edm, a.rap, a.metal);
let mut c = Color(255, 0, 0);
println!("Color: {} {} {}", c.0, c.1, c.2);
 */
let mut p = Plant::new("centaurea cyanus", "Germany", "Stimulates liver and gallbladder, aids with fever and chest congestion");
println!("Plant {} {} {}", p.name, p.origin, p.properties);
}