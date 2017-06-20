use image::{Rgb, Pixel, Primitive, ImageBuffer, GenericImage, DynamicImage};
use std::collections::HashMap;
use std::path::Path;
use std::io;

fn delta<S: Primitive, T: Pixel<Subpixel=S>>(x: &T, y: &T) -> u32 
    where S: Into<u32>
{
    // Euclidean distance square
    // https://en.wikipedia.org/wiki/Color_difference#Euclidean
    // TODO: incorporate alpha values? s/rgb/rgba ?
    x.to_rgb().data.into_iter()
        .zip(y.to_rgb().data.into_iter())
        .map(|(&a,&b)| if a>b { (a,b) } else { (b,a) } )    // avoid negatives
        .map(|(a,b)| (a.into(),b.into()))                   // avoid overflow
        .map(|(a,b)| (a-b)*(a-b))
        .sum()
}

fn nearest_color(c: &Rgb<u8>) -> &'static Color {
    COLORS_MAP.iter()
        .map(|(name,rgb)| (name, delta(c, &rgb)))
        .min_by(|&(_,d1),&(_,d2)| d1.cmp(&d2))
        .unwrap()   // only panics if COLORS_MAP.is_empty() 
        .0
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColorGrid(Vec<Vec<&'static Color>>);  // TODO: Option<Color> ?

impl ColorGrid {
    pub fn from_image(input: &DynamicImage) -> Self {
        let pixels: Vec<_> = input.pixels().collect();
        ColorGrid(pixels.chunks(input.width() as usize)
            .map(|row| row.iter()
                 .map(|&(_,_,rgba)| nearest_color(&rgba.to_rgb()))
                 .collect())
            .collect())
    }
    pub fn _from_image_2(input: &DynamicImage) -> Self {
        let (w,h) = (input.width() as usize, input.height() as usize);
        let mut grid: Vec<_> = (0..h).map(|_| Vec::with_capacity(w)).collect();
        for (_,y,rgba) in input.pixels() {
            grid[y as usize].push(nearest_color(&rgba.to_rgb()));
        }
        ColorGrid(grid)
    }
    fn get(&self, x: usize, y: usize) -> Option<&'static Color> {
        self.0.get(y).and_then(|row| row.get(x).map(|&c| c))
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn export<P: AsRef<Path>>(&self, out: P) -> io::Result<()> {
        let (w, h) = (self.width() as u32, self.height() as u32);
        ImageBuffer::from_fn(w, h, |x,y| { 
            let color = self.get(x as usize, y as usize).unwrap();
            COLORS_MAP[color]
        }).save(out)
    }
}

// http://lego.wikia.com/wiki/Colour_Palette
// pattern = '<td><a href="/wiki/(\w+)" title="[^"]+">[^"]+</a>\n"
// pattern += "</td><td>[^"]+\n</td><td style="background:#(\w+)">'
// re.findall(pattern, open("Colour_Palette", 'r').read())

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    BrickYellow,
    Nougat,
    BrightRed,
    BrightBlue,
    BrightYellow,
    Black,
    DarkGreen,
    BrightGreen,
    DarkOrange,
    MediumBlue,
    BrightOrange,
    BrightReddishViolet,
    SandBlue,
    SandYellow,
    EarthBlue,
    EarthGreen,
    SandGreen,
    DarkRed,
    FlameYellowishOrange,
    ReddishBrown,
    MediumStoneGrey,
    DarkStoneGrey,
    LightStoneGrey,
    LightRoyalBlue,
    BrightPurple,
    LightPurple,
    CoolYellow,
    MediumLilac,
    LightNougat,
    DarkBrown,
    MediumNougat,
    MediumAzur,
    Aqua,
    Lavender,
    WhiteGlow,
    SpringYellowishGreen,
    OliveGreen,
    Silver,
    MetallicDarkGrey,
    PhosphorescentGreen,
    WarmGold
}

lazy_static! {
    static ref COLORS_MAP: HashMap<Color, Rgb<u8>> = {
        use self::Color::*;
        // treat all lego colors as 0% opacity
        fn fc(r: u8, g: u8, b: u8) -> Rgb<u8> { Rgb::from_channels(r,g,b,255) }
        let mut map = HashMap::new();
        map.insert(White,           fc(255, 255, 255));
        map.insert(BrickYellow,     fc(217, 187, 123));
        map.insert(Nougat,          fc(214, 114,  64));
        map.insert(BrightRed,       fc(255,   0,   0));
        map.insert(BrightBlue,      fc(  0,   0, 255));
        map.insert(BrightYellow,    fc(255, 255,   0));
        map.insert(Black,           fc(  0,   0,   0));
        map.insert(DarkGreen,       fc(  0, 153,   0));
        map.insert(BrightGreen,     fc(  0, 204,   0));
        map.insert(DarkOrange,      fc(168,  61,  21));
        map.insert(MediumBlue,      fc( 71, 140, 198));
        map.insert(BrightOrange,    fc(255, 102,   0));
        map.insert(BrightReddishViolet, fc(153, 0, 102));
        map.insert(SandBlue,        fc( 94, 116, 140));
        map.insert(SandYellow,      fc(141, 116,  82));
        map.insert(EarthBlue,       fc(  0,  37,  65));
        map.insert(EarthGreen,      fc(  0,  51,   0));
        map.insert(SandGreen,       fc( 95, 130, 101));
        map.insert(DarkRed,         fc(128,   8,  27));
        map.insert(FlameYellowishOrange, fc(244, 155, 0));
        map.insert(ReddishBrown,    fc( 91,  28,  12));
        map.insert(MediumStoneGrey, fc(156, 146, 145));
        map.insert(DarkStoneGrey,   fc( 76,  81,  86));
        map.insert(LightStoneGrey,  fc(228, 228, 218));
        map.insert(LightRoyalBlue,  fc(135, 192, 234));
        map.insert(BrightPurple,    fc(222,  55, 139));
        map.insert(LightPurple,     fc(238, 157, 195));
        map.insert(CoolYellow,      fc(255, 255, 153));
        map.insert(MediumLilac,     fc( 44,  21, 119));
        map.insert(LightNougat,     fc(245, 193, 137));
        map.insert(DarkBrown,       fc( 48,  15,   6));
        map.insert(MediumNougat,    fc(170, 125,  85));
        map.insert(MediumAzur,      fc(104, 195, 226));
        map.insert(Aqua,            fc(211, 242, 234));
        map.insert(Lavender,        fc(205, 164, 222));
        map.insert(WhiteGlow,       fc(245, 243, 215));
        map.insert(SpringYellowishGreen, fc(226, 249, 154));
        map.insert(OliveGreen,      fc(119, 119,  78));
        map.insert(Silver,          fc(141, 148, 150));
        map.insert(MetallicDarkGrey, fc(73,  63,  59));
        map.insert(PhosphorescentGreen, fc(254, 252, 213));
        map.insert(WarmGold,        fc(170, 127,  46));
        map
    };
}

