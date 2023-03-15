use drawable::shape::Shape;
use drawable::shape_group::ShapeGroup;
use drawable::{DrawUniforms, Drawable};
use glium::{Display, DrawParameters, Frame, Program};
use rand::{thread_rng, Rng};
use util::attribute::Attr;
use util::bufferable::Bufferable;
use util::Manipulate;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const SIZE: usize = WIDTH * HEIGHT;

pub struct GameOfLife {
    shapegroup: ShapeGroup<Shape>,
    grid: Vec<i32>,
    scl: f32,
}

#[inline]
fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1))
}

impl GameOfLife {
    pub fn default(display: &Display) -> Self {
        let mut shapegroup = ShapeGroup::default();
        let mut attributes = vec![];
        let mut grid = vec![0; SIZE];
        let dim = display.get_framebuffer_dimensions();
        let dim = (dim.0 as usize, dim.1 as usize);
        let scl = (dim.0 / 20) as f32;
        let quad = Shape::quad(display, scl);
        for i in 0..SIZE {
            let mut attr = Attr::default();

            let mut x = (i % WIDTH) as f32;
            let mut y = (i / WIDTH) as f32;

            x = map(x, 0.0, WIDTH as f32, -2.0, 2.0);
            y = map(y, 0.0, HEIGHT as f32, -2.0, 2.0);

            attr.world_position = [x, y, -1.0];

            let rand: bool = thread_rng().gen_bool(0.5);
            if rand {
                attr.color = [0.0, 0.0, 0.0, 1.0];
                grid[i] = 0;
            } else {
                attr.color = [1.0; 4];
                grid[i] = 1;
            }
            attributes.push(attr);
        }

        let attributes = Attr::new_vbo(display, &attributes);
        shapegroup.push((quad, attributes));

        GameOfLife {
            shapegroup,
            grid,
            scl,
        }
    }
}

impl Drawable for GameOfLife {
    fn draw(
        &self,
        target: &mut Frame,
        program: &Program,
        params: &DrawParameters,
        uniforms: DrawUniforms,
    ) {
        self.shapegroup.draw(target, program, params, uniforms);
    }

    fn update(&mut self) {}
}

impl Manipulate for GameOfLife {
    fn rotate_axis(&mut self, axis: usize, ang: f32) {
        todo!()
    }
}
