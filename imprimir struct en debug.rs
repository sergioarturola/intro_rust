// ' :? ' hace referencia que queremos imprimir algo en modo debug, lo cual es util para saber
// que valores esta tomando la estructura, tambien es necesario agregar la directiva #[derive(Debug)]
#[derive(Debug)]
struct Rectangulo{
	base: u32,
	altura: u32,
}


fn main() {

	//variable de la estrucutura
	let rec_uno = Rectangulo{
		base: 32,
		altura: 10,
	};

	println!("El rectangulo es: {:?}",rec_uno);


}

