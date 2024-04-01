use inquire::{error::InquireError, CustomType, Select};
use std::fs;

fn main() {
    interactive();
}

fn interactive() {
    let ans: Result<&str, InquireError> = Select::new(
        "Quel modèle ?",
        vec!["Baggi", "Masu", "Soda Masu", "Porte-Cartes"],
    )
    .prompt();

    match ans {
        Ok(choice) => match choice {
            "Baggi" => interactive_baggi(),
            "Masu" => interactive_masu(),
            "Soda Masu" => interactive_masusoda(),
            "Porte-Cartes" => interactive_vfold(),
            _ => eprintln!("Merci de recommencer"),
        },
        Err(_) => eprintln!("Merci de recommencer"),
    }
}

fn interactive_baggi() {
    let length: f32 = CustomType::new("Longueur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la longueur en mm (séparateur .)")
        .prompt()
        .unwrap();
    let width: f32 = CustomType::new("Largeur/Hauteur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la largeur/hauteur en mm (séparateur .)")
        .prompt()
        .unwrap();
    fs::write(
        format!(r#"Baggi-{}-{}.svg"#, length, width),
        pattern_baggi(length, width),
    )
    .expect("Problème d'écriture");
}

fn interactive_masu() {
    let length: f32 = CustomType::new("Côté:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la longueur en mm (séparateur .)")
        .prompt()
        .unwrap();
    fs::write(
        format!(r#"Masu-{}.svg"#, length),
        pattern_masu("Masu", length, length, length / 2.),
    )
    .expect("Problème d'écriture");
}

fn interactive_masusoda() {
    let length: f32 = CustomType::new("Longueur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la longueur en mm (séparateur .)")
        .prompt()
        .unwrap();
    let width: f32 = CustomType::new("Largeur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la largeur en mm (séparateur .)")
        .prompt()
        .unwrap();
    let height: f32 = CustomType::new("Hauteur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la hauteur en mm (séparateur .)")
        .prompt()
        .unwrap();
    fs::write(
        format!(r#"SodaMasu-{}-{}-{}.svg"#, length, width, height),
        pattern_masu("SodaMasu", length, width, height),
    )
    .expect("Problème d'écriture");
}

fn interactive_vfold() {
    let width: f32 = CustomType::new("Largeur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez la largeur en mm (séparateur .)")
        .prompt()
        .unwrap();
    let thickness: f32 = CustomType::new("Epaisseur:")
        .with_formatter(&|i: f32| format!("{i}"))
        .with_error_message("Entrez un nombre valide")
        .with_help_message("Entrez l'épaisseur en mm (séparateur .)")
        .prompt()
        .unwrap();
    fs::write(
        format!(r#"CardHolder-{}-{}.svg"#, width, thickness),
        pattern_vfold(width, thickness),
    )
    .expect("Problème d'écriture");
}

fn pattern_vfold(width: f32, thickness: f32) -> String {
    return format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
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
    return format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
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
        B = (length + 2. * height) / r2,
        C = (length + 4. * height) / r2,
        X = cote - (length + 4. * height) / r2,
        Y = cote - (length + 2. * height) / r2,
        Z = cote - length / r2,
        s = 0.1
    );
}

fn pattern_baggi(length: f32, width: f32) -> String {
    return format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg
	xmlns="http://www.w3.org/2000/svg"
	xmlns:svg="http://www.w3.org/2000/svg"
	id="Baggi-{l}x{w}" version="1.1"
	width="{C}mm" height="{H}mm"
	viewBox="0 0 {C} {H}">
	<title>Baggi:{l}x{w}</title>
	<g id="Decoupe">
		<rect id="Bordure"
			style="fill:none;stroke:black;stroke-width:{s}"
			x="0" y="0"
			width="{C}" height="{H}"
		/>
	</g>
	<g id="Rainurage">
		<path id="One"
			d="M0,{y1} h{C} M0,{y2} h{C} M0,{y3} h{C}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
		<path id="Two"
			d="M0,0 l{w},{w} M0,{y1} l{w},{w} l-{w},{w} l{w},{w}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
		<path id="Three"
			d="M{C},0 l-{w},{w} M{C},{y1} l-{w},{w} l{w},{w} l-{w},{w}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>
		<path id="Four"
			d="M{w},0 v{H} M{ww},0 v{H}"
			style="fill:none;stroke:blue;stroke-width:{s}"
		/>	
	</g>
</svg>"#,
        s = 0.1,
        l = length,
        w = width,
        H = 4. * width,
        C = 2. * width + length,
        ww = width + length,
        y1 = width,
        y2 = 2. * width,
        y3 = 3. * width
    );
}
