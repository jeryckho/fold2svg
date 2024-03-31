use std::env;

fn main() {
	// Récupération des arguments de la ligne de commande
	let args: Vec<String> = env::args().collect();

	// Vérification que deux arguments ont été fournis
	if args.len() != 3 {
		println!("Veuillez fournir exactement deux nombres en argument !");
		return;
	}

	// Conversion des arguments en nombres
	let width: f32 = match args[1].parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Le premier argument n'est pas un nombre valide !");
			return;
		}
	};
	let thickness: f32 = match args[2].parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Le deuxième argument n'est pas un nombre valide !");
			return;
		}
	};

	// println!("{}", pattern_vfold(width, thickness));
	pattern_vfold(width, thickness);
	let res = pattern_masu("SodaMasu", 70., 70., 35.);
	println!("{}", res);
}

fn pattern_vfold(width: f32, thickness: f32) -> String {
	return format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
	xmlns="http://www.w3.org/2000/svg"
	xmlns:svg="http://www.w3.org/2000/svg"
	id="VFold-{h}x{t}" version="1.1"
	width="{l}mm" height="{h}mm"
	viewBox="0 0 {l} {h}">
	<title>VFold:{h}x{t}</title>
	<g id="Decoupe">
		<rect id="Bordure"
			style="fill:none;stroke:black;stroke-width:0.1"
			x="0" y="0"
			width="{l}" height="{h}"
		/>
	</g>
	<g id="Rainurage">
		<path id="Pliure"
			d="M{p1},0 v{h} M{p2},0 v{h} l{h},-{h} v{h} M{p3},0 v{h} M{p4},0 v{h} M{p5},{h} v-{h} l{h},{h}"
			style="fill:none;stroke:blue;stroke-width:0.1"
		/>
	</g>
</svg>"#,
		h = width,
		t = thickness,
		l = 4. * width + 2. * thickness,
		p1 = width - thickness,
		p2 = width,
		p3 = 2. * width + thickness,
		p4 = 3. * width + thickness,
		p5 = 3. * width + 2. * thickness
	);
}

fn pattern_masu(name: &str, length: f32, width: f32, height: f32) -> String {
	let diag: f32 = 4. * height + width + length;
	let r2 = f64::sqrt(2.) as f32;
	let cote: f32 = diag / r2;
	return format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
	xmlns="http://www.w3.org/2000/svg"
	xmlns:svg="http://www.w3.org/2000/svg"
	id="{n}-{l}x{w}x{h}" version="1.1"
	width="{c}mm" height="{c}mm"
	viewBox="0 0 {c} {c}">
	<title>{n}:{l}x{w}x{h}</title>
	<g id="Decoupe">
		<rect id="Bordure"
			style="fill:none;stroke:black;stroke-width:{s}"
			x="0" y="0"
			width="{c}" height="{c}"
		/>
	</g>
	<g id="Rainurage">
		<path id="One"
			d="M0,{A} L{A},0 L{c},{Z} L{Z},{c} L0,{A}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
		<path id="Two"
			d="M0,{B} L{B},0 L{c},{Y} L{Y},{c} L0,{B}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
		<path id="Three"
			d="M0,{C} L{C},0 L{c},{X} L{X},{c} L0,{C}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
		<path id="Four"
			d="M0,{B} h{d} M{B},0 v{d} M{Y},{c} v-{d} M{c},{Y} h-{d}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
	</g>
</svg>"#,
		n = name,
		c = cote,
		h = height,
		w = width,
		l = length,
		d = height * r2,
		A = length / r2,
		B = (length +2.*height)/r2,
		C = (length+4.*height)/r2,
		X = cote - (length+4.*height)/r2,
		Y = cote - (length +2.*height)/r2,
		Z = cote - length / r2,
		s = 0.1
	);
}
