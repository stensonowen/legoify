
use image::{Pixel, Primitive};
use phf;

pub fn delta<S: Primitive, T: Pixel<Subpixel=S>>(x: T, y: T) -> S {
    // Euclidean distance square
    // https://en.wikipedia.org/wiki/Color_difference#Euclidean
    // TODO: incorporate alpha values?
    x.to_rgb().data.into_iter()
        .zip(y.to_rgb().data.into_iter())
        .map(|(&a,&b)| if a>b { (a,b) } else { (b,a) } )
        .map(|(a,b)| (a-b)*(a-b))
        .fold(S::zero(), |acc,i| acc+i)
}

// http://lego.wikia.com/wiki/Colour_Palette
// pattern = '<td><a href="/wiki/(\w+)" title="[^"]+">[^"]+</a>\n</td><td>[^"]+\n</td><td style="background:#(\w+)">'
// re.findall(pattern, open("Colour_Palette", 'r').read())
pub static COLORS: phf::Map<&'static str, (u8,u8,u8)> = phf_map! {
    "White" => (255, 255, 255), 
    "Brick_Yellow" => (217, 187, 123),
    "Nougat" => (214, 114, 64),
    "Bright_Red" => (255, 0, 0),
    "Bright_Blue" => (0, 0, 255),
    "Bright_Yellow" => (255, 255, 0),
    "Black" => (0, 0, 0),
    "Dark_Green" => (0, 153, 0),
    "Bright_Green" => (0, 204, 0),
    "Dark_Orange" => (168, 61, 21),
    "Medium_Blue" => (71, 140, 198),
    "Bright_Orange" => (255, 102, 0),
    "Bright_Reddish_Violet" => (153, 0, 102),
    "Sand_Blue" => (94, 116, 140),
    "Sand_Yellow" => (141, 116, 82),
    "Earth_Blue" => (0, 37, 65),
    "Earth_Green" => (0, 51, 0),
    "Sand_Green" => (95, 130, 101),
    "Dark_Red" => (128, 8, 27),
    "Flame_Yellowish_Orange" => (244, 155, 0),
    "Reddish_Brown" => (91, 28, 12),
    "Medium_Stone_Grey" => (156, 146, 145),
    "Dark_Stone_Grey" => (76, 81, 86),
    "Light_Stone_Grey" => (228, 228, 218),
    "Light_Royal_Blue" => (135, 192, 234),
    "Bright_Purple" => (222, 55, 139),
    "Light_Purple" => (238, 157, 195),
    "Cool_Yellow" => (255, 255, 153),
    "Medium_Lilac" => (44, 21, 119),
    "Light_Nougat" => (245, 193, 137),
    "Dark_Brown" => (48, 15, 6),
    "Medium_Nougat" => (170, 125, 85),
    "Medium_Azur" => (104, 195, 226),
    "Aqua" => (211, 242, 234),
    "Lavender" => (205, 164, 222),
    "White_Glow" => (245, 243, 215),
    "Spring_Yellowish_Green" => (226, 249, 154),
    "Olive_Green" => (119, 119, 78),
    "Silver" => (141, 148, 150),
    "Metallic_Dark_Grey" => (73, 63, 59),
    "Phosphorescent_Green" => (254, 252, 213),
    "Warm_Gold" => (170, 127, 46),
};


