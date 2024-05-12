use bevy::prelude::*;

/// Something that can overwrite a value, typically a Bundle.
pub trait Class<B> {
    fn apply(self, b: &mut B);
}

impl<T> Class<T> for () {
    fn apply(self, _b: &mut T) {}
}

impl<F, B> Class<B> for F
where
    F: FnOnce(&mut B),
{
    fn apply(self, b: &mut B) {
        self(b);
    }
}

impl<F1, F2, B> Class<B> for (F1, F2)
where
    F1: Class<B>,
    F2: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
    }
}

impl<F1, F2, F3, B> Class<B> for (F1, F2, F3)
where
    F1: Class<B>,
    F2: Class<B>,
    F3: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
        self.2.apply(b);
    }
}

impl<F1, F2, F3, F4, B> Class<B> for (F1, F2, F3, F4)
where
    F1: Class<B>,
    F2: Class<B>,
    F3: Class<B>,
    F4: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
        self.2.apply(b);
        self.3.apply(b);
    }
}

impl Class<NodeBundle> for NodeBundle {
    fn apply(self, b: &mut NodeBundle) {
        *b = self;
    }
}

impl Class<ImageBundle> for ImageBundle {
    fn apply(self, b: &mut ImageBundle) {
        *b = self;
    }
}

/// Something that can overwrite a value, typically a node bundle.
/// Depends on an [`AssetServer`], unlike [`Class`].
pub trait AssetClass<B> {
    fn apply(self, assets: &AssetServer, b: &mut B);
}

impl<T> AssetClass<T> for () {
    fn apply(self, _a: &AssetServer, _b: &mut T) {}
}

impl<F, B> AssetClass<B> for F
where
    F: FnOnce(&AssetServer, &mut B),
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self(a, b);
    }
}

impl<F1, F2, B> AssetClass<B> for (F1, F2)
where
    F1: AssetClass<B>,
    F2: AssetClass<B>,
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
    }
}

impl<F1, F2, F3, B> AssetClass<B> for (F1, F2, F3)
where
    F1: AssetClass<B>,
    F2: AssetClass<B>,
    F3: AssetClass<B>,
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
        self.2.apply(a, b);
    }
}

impl<F1, F2, F3, F4, B> AssetClass<B> for (F1, F2, F3, F4)
where
    F1: AssetClass<B>,
    F2: AssetClass<B>,
    F3: AssetClass<B>,
    F4: AssetClass<B>,
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
        self.2.apply(a, b);
        self.3.apply(a, b);
    }
}

impl AssetClass<ButtonBundle> for ButtonBundle {
    fn apply(self, _a: &AssetServer, b: &mut ButtonBundle) {
        *b = self;
    }
}

impl AssetClass<TextBundle> for TextBundle {
    fn apply(self, _a: &AssetServer, b: &mut TextBundle) {
        *b = self;
    }
}

impl AssetClass<TextStyle> for TextStyle {
    fn apply(self, _a: &AssetServer, b: &mut TextStyle) {
        *b = self;
    }
}