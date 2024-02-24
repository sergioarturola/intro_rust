//estrucutura que representa el rectangulo
struct Rectangulo{
	base: u32,
	altura: u32,
}


fn main() {

	//instancia de la estrucutura
	let rec_uno = Rectangulo{
		base: 32,
		altura: 10,
	};

	println!("El area del rectangulo es: {}", area_rectangulo(&rec_uno));

}

//recibiendo como parametro un dato del tip "Rectangulo"
fn area_rectangulo(rectangulo: &Rectangulo)-> u32{//toma prestado el valor "rec_uno"
	rectangulo.base*rectangulo.altura
}

