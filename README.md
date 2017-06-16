# legoify

### requirements
* image type must be supported by [PistonDevelopers/image](https://crates.io/crates/image)
* (?) image type must have transparent background (unless we can infer it?)

### steps
1. Simplify color palette. This should be equivalent to GIMP's Image -> Mode -> Indexed with a custom palette of Lego colors. This should be linear in the number of pixels in an image.
2. Pixellate image into blocks. This is basically reducing an image's resolution. Should also be linear in the number of pixels.
3. Break pixel groups into legos. Try to split into lego sizes based on proportion (e.g. use mostly 2x4's and few 1x1's). This might turn into the knapsack problem (if we're lucky-ish).

### why
I have done no research but I'm confident this hasn't been done right. I believe a proper solution is right in that sweet spot of too gimmicky for people with self respect and too maybe-NP-complete for people who just care about legos. We'll see.


