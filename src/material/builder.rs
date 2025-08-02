use std::rc::Rc;

use crate::Color;
use crate::material::Material;
use crate::pattern::Pattern;
use crate::pattern::solid::SolidPattern;

pub struct MaterialBuilder {
    pattern: Rc<dyn Pattern>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl MaterialBuilder {
    pub fn new() -> Self {
        Self {
            pattern: Rc::new(SolidPattern::new(Color::white())),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.pattern = Rc::new(SolidPattern::new(color));

        self
    }

    #[allow(dead_code)]
    pub fn with_pattern(mut self, pattern: Rc<dyn Pattern>) -> Self {
        self.pattern = pattern;

        self
    }

    #[allow(dead_code)]
    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;

        self
    }

    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;

        self
    }

    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;

        self
    }

    #[allow(dead_code)]
    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;

        self
    }

    pub fn build(self) -> Material {
        Material {
            pattern: self.pattern,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
        }
    }
}
