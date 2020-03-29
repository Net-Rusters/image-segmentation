use image::*;
use std::convert::TryInto;

#[derive(Debug)]
struct Graph {
    list: Vec< Vec<u32> >,
    labels: Vec<u32>,
    label_count: u32,
}

impl Graph {
    fn new(width: &u32, height: &u32) -> Graph {
        let mut list: Vec< Vec<u32> > = vec![vec![]; (width*height).try_into().unwrap() ];
        let mut labels: Vec<u32> = vec![0u32; (width*height).try_into().unwrap()];
        let mut label_count: u32 = 0u32;
        return Graph {list, labels, label_count};
    }

    fn add_edge(&mut self, from: &u32, to: &u32) {
        self.list[(*from) as usize].push(*to);
    }

    fn get_label(&self, index: &u32) -> u32 {
        self.labels[(*index) as usize]
    }

    fn set_label(&mut self, index: &u32, label: &u32) {
        self.labels[(*index) as usize] = *label;
    }

    fn dfs(&mut self, now: &u32, label: &u32) {
        self.set_label(&now, &label);
        let list = (self.list[(*now) as usize]).clone(); 
        for other in list {
            if self.get_label(&other) == 0u32 {
                self.dfs(&other, &label);
            }
        }
    }

    fn label_all(&mut self) {
        let mut label : u32 = 1u32;
        for i in 0..self.list.len() {
            if self.get_label( &(i as u32) ) == 0 {
                self.dfs(&(i as u32), &label);
                label += 1u32;
            }
        }
        self.label_count = label;
    }
}

fn to_pair(value: &u32, max_y: &u32) -> Pair {
    Pair { x: value / max_y,  y: value % max_y }
}

fn linearize(x: &u32, y: &u32, max_y: &u32) -> u32 {
    x*max_y + y
}

fn compare(pix1: &image::Rgba<u8>, pix2: &image::Rgba<u8>) -> bool {
    let channels1 = pix1.channels();
    let channels2 = pix2.channels();

    let rmean: f64 = (channels1[0] as f64 - channels2[0] as f64) / 2f64;
    let r: f64 = (channels1[0] as f64 - channels2[0] as f64);
    let g: f64 = (channels1[1] as f64 - channels2[1] as f64);
    let b: f64 = (channels1[2] as f64 - channels2[2] as f64);

    let mut dist : f64 = (((512f64+rmean)*r*r)/256f64) + 4f64*g*g + (((767f64-rmean)*b*b)/256f64);
    dist = dist.sqrt();
    if dist < 50f64 {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let mut image1 = image::open("image.jpg").unwrap();

    let mut image2: image::RgbImage = image::ImageBuffer::new(100, 100);

    image1 = image1.resize(100u32, 100u32, image::imageops::FilterType::Gaussian);
    
    let (height, width) = image1.dimensions();

    let mut graph = Graph::new(&width, &height);

    let mut labels: Vec< Vec<i32> > = vec![vec![-1; height.try_into().unwrap()]; width.try_into().unwrap()];

    let mut pix = image1.get_pixel(0, 0);

    for i in 0..height {
        for j in 0..width {
            let mut pixel = image1.get_pixel(i, j);
            if i != 0  {
               let mut pixel2 = image1.get_pixel(i-1, j);
                if compare(&pixel, &pixel2) {
                    graph.add_edge(&linearize(&i, &j, &width), &linearize(&(i-1), &j, &width));
                }
            }
            if j != 0 {
                let mut pixel2 = image1.get_pixel(i, j-1);
                if compare(&pixel, &pixel2) {
                    graph.add_edge(&linearize(&i, &j, &width), &linearize(&i, &(j-1), &width));
                }
            }
            if i != height-1  {
                let mut pixel2 = image1.get_pixel(i+1, j);
                 if compare(&pixel, &pixel2) {
                    graph.add_edge(&linearize(&i, &j, &width), &linearize(&(i+1), &j, &width));
                 }
             }
             if j != width-1 {
                 let mut pixel2 = image1.get_pixel(i, j+1);
                 if compare(&pixel, &pixel2) {
                    graph.add_edge(&linearize(&i, &j, &width), &linearize(&i, &(j+1), &width));
                 }
             }
       }
    }
    graph.label_all();

    let colors: Vec<(u8, u8, u8)> = vec![(0,0,0), (255,255,255), (255,0,0), (0,255,0), (0,0,255), 
                                        (0,255,255), (255,0,255), (192,192,192), (128,128,128),
                                        (128,0,0), 	(128,128,0), (0,128,0), (128,0,128),
                                        (255,228,225), (188,143,143), (210,105,30)];
    
    let mut labelcount1 = 0;
    let mut labelcount2 = 0;
    
    for i in 0..height {
        for j in 0..width {
            let mut pixel = *image2.get_pixel(i,j);
            let mut channels = pixel.channels_mut();
            let label = graph.get_label(&linearize(&i, &j, &width)) % 16u32;
            let chan = colors[label as usize];
            channels[0] = chan.0;
            channels[1] = chan.1;
            channels[2] = chan.2;
            image2.put_pixel(i, j, pixel);
        }
    }
    image2.save("test2.jpg").unwrap();
    println!("{}", graph.label_count);
}