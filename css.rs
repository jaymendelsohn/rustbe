struct Stylesheet {
	rules: Vec<Rule>,
}

struct Rule {
	selectors: Vec<Selector>,
	declarations: Vec<Declaration>,
}

enum Selector {
	Simple(SimpleSelector),
}

struct SimpleSelector {
	tag_name: Option<String>,
	id: Option<String>,
	class: Vec<String>,
}

struct Declaration {
	name: String,
	value: Value,
}

enum Value {
	Keyword(String),
	Length(f32, Unit),
	ColorValue(Color),
	//TODO Insert more values

}

enum Unit {
	Px,
}

struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

//TODO Parser, etc
