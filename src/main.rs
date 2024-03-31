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

	let svg = format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
	xmlns="http://www.w3.org/2000/svg"
	xmlns:svg="http://www.w3.org/2000/svg"
	id="Pliage{h}x{t}" version="1.1"
	width="{l}mm" height="{h}mm"
	viewBox="0 0 {l} {h}">
	<title>{h}x{t}</title>
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
</svg>"#, h=width, t=thickness, l=4.*width+2.*thickness, p1=width-thickness, p2=width, p3=2.*width+thickness, p4=3.*width+thickness, p5=3.*width+2.*thickness);

	println!("{}", svg);
}
