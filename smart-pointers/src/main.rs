//smart pointers

use std::ops::Deref;

fn main() {
    //referencias (&)

    let x = 5;
    let y = &x;

    //reference counter (Contador de referencias)
    //String y Vec<T>

    /*Smart pointer son usualmente implementados usando struts
        pero implementando los traits Deref y Drop
        Deref permite a las instancias de msart pointer comportarse como referencias
        pero que el mismo codigo que funciona con referencias, funcione con smart pointers
        Drop trait permite definir logica que se ejecute una vez que el smart pointer sale del scoope
    */

    //Box<T> = caja
    let x = 2;
    let y = Box::new(2);
    println!("{}", y);

    //Linked list = listas enlazadas
    //(valor, nodo1) -> (valor2, nodo2) -> (valor3, null)

    enum List{
        Node(i32, Box<List>),
        None
    }

    let node3 = List::Node(10, Box::new(List::None));
    let node2 = List::Node(2, Box::new(node3));
    let node1 = List::Node(3, Box::new(node2));

    //Defer trait: hacer posible la dereferencia (*)
    let x = 5;
    let y = &x;
    let z = &y;

    let w = MiCaja::new(x);

    if x == 5{
        println!("hola")
    }
    if *y == 5{
        println!("hola")
    }
    if **z == 5{
        println!("hola")
    }
    if *w == 5{
        println!("hola")
    }

    //Drop trait = que hacer cuando la instancia dale del scope

    //Rerence Counted Smart Pointer: permite que un valor tenga muchos dueños
    /*
        Usamos Rc cuando queremos asignar datos en el heap para que sea accedido
        en multiples partes del codigo, y no podemos determinar en tiempo de 
        compilacion el ultimo que accedera a estos datos. Si supieramos de antemano
        pero no lo sabemos, podriamos hacer que ese ultimo sea el dueño, 
        quien seria el ultimo. Entonces Rc lleva un contador de referencias con todos 
        los dueños, y cunado ya no quedan dueños, puede limpiar el dato
     */

    use std::rc::Rc;

    enum List2 {
        Node(i32, Rc<List2>),
        None
    }

    //node1 ->
    //          node2-> node3->None
    //node0 ->

    let node_l3 = List2::Node(10, Rc::new(List2::None));
    let node_l2 = List2::Node(2, Rc::new(node_l3));
    let node_l2_rc = Rc::new(node_l2);
    {
        println!("Numero de refs {}",Rc::strong_count(&node_l2_rc));
        let node_l1 = List2::Node(3, Rc::clone(&node_l2_rc));
        println!("Numero de refs {}",Rc::strong_count(&node_l2_rc));
        let node_l0 = List2::Node(5, Rc::clone(&node_l2_rc));
    }
    println!("Numero de refs {}",Rc::strong_count(&node_l2_rc));

}

struct MiCaja<T>(T);

impl <T> MiCaja<T> {
    fn new(x: T) -> MiCaja<T>{
        MiCaja(x)
    }
}

impl<T> Deref for MiCaja<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//Drop trait = que hacer cuando la instancia dale del scope
impl<T> Drop for MiCaja<T> {
    fn drop(&mut self) {
        println!("Adios :c")
    }
}