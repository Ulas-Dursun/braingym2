mod ders1;
mod ders2;
mod ders3;
mod ders4;
mod ders5;
/*use std::io;
use std::rc::Rc;
use std::cell::RefCell;

// Bağlı liste düğümü (node) yapısı
#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

// Bağlı liste yapısı
struct LinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
}

impl LinkedList {
    // Yeni bir bağlı liste oluşturur
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Listeye eleman ekler
    fn push(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(ListNode {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    // Listenin uzunluğunu hesaplar
    fn length(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            count += 1;
            current = node.borrow().next.clone();
        }
        count
    }

    // n. elemanı listeden çıkarır
    fn remove_nth_from_end(&mut self, n: usize) {
        let length = self.length();

        if n > length {
            println!("Geçersiz konum!");
            return;
        }

        let mut dummy = Rc::new(RefCell::new(ListNode {
            value: 0,
            next: self.head.clone(),
        }));
        let mut current = dummy.clone();

        // Silinecek elemanın bir öncesine kadar ilerle
        for _ in 0..(length - n) {
            let next = current.borrow().next.clone().unwrap();
            current = next;
        }

        // n. elemanı atla ve listenin geri kalanını bağla
        let next = current.borrow().next.clone().unwrap();
        current.borrow_mut().next = next.borrow().next.clone();

        // Yeni baş düğümü güncelle
        self.head = dummy.borrow().next.clone();
    }

    // Listeyi yazdırır
    fn print_list(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} => ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}

fn main() {
    // Kullanıcıdan input al
    let mut input = String::new();
    println!("Bağlı liste elemanlarını girin (boşluk ile ayırın):");
    io::stdin().read_line(&mut input).expect("Başarısız okuma");
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Geçersiz sayı"))
        .collect();

    let mut list = LinkedList::new();
    for value in values.into_iter().rev() {
        list.push(value);
    }

    println!("Silmek istediğiniz elemanın konumunu girin:");
    let mut nth = String::new();
    io::stdin().read_line(&mut nth).expect("Başarısız okuma");
    let nth: usize = nth.trim().parse().expect("Geçersiz sayı");

    list.remove_nth_from_end(nth);
    list.print_list();
}*/

/*use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode { value, next: None }))
    }
}

fn remove_duplicates(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if head.is_none() {
        return head;
    }

    let mut current = head.clone();
    let mut prev: Option<Rc<RefCell<ListNode>>> = None; // Explicit type annotation here
    let mut seen = HashSet::new();

    while let Some(node) = current.clone() {
        let node_value = node.borrow().value;

        if seen.contains(&node_value) {
            // If it's a duplicate, bypass the current node
            if let Some(prev_node) = prev.clone() {
                prev_node.borrow_mut().next = node.borrow().next.clone();
            }
        } else {
            // Add unique value to the set
            seen.insert(node_value);
            prev = current.clone();
        }

        current = node.borrow().next.clone();
    }

    head
}

fn print_linked_list(head: Option<Rc<RefCell<ListNode>>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{}", node.borrow().value);
        current = node.borrow().next.clone();
        if current.is_some() {
            print!(" => ");
        }
    }
    println!();
}

fn create_linked_list(elements: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let mut head: Option<Rc<RefCell<ListNode>>> = None;
    let mut current: Option<Rc<RefCell<ListNode>>> = None;

    for element in elements.into_iter().rev() {
        let new_node = ListNode::new(element);
        new_node.borrow_mut().next = head.clone();
        head = Some(new_node.clone());
        if current.is_none() {
            current = Some(new_node);
        }
    }

    head
}

fn main() {
    let elements = vec![1, 2, 3, 4, 2, 1, 5, 13, 13, 5];
    let head = create_linked_list(elements);
    println!("Original list:");
    print_linked_list(head.clone());

    // Remove duplicates
    let head = remove_duplicates(head);
    println!("List after removing duplicates:");
    print_linked_list(head);
}*/

// Importing the Rc (Reference Counted) smart pointer
/*use std::rc::Rc;

// Function to demonstrate stack memory
fn stack_memory_example() {
    // Local variables are stored on the stack
    let x = 42; // `x` is a stack variable with a fixed size and known at compile time
    let y = 3.14; // `y` is another stack variable

    println!("Stack variable x: {}", x);
    println!("Stack variable y: {}", y);
}

// Function to demonstrate heap memory
fn heap_memory_example() {
    // Allocate a value on the heap and create a `Box` to manage it
    let heap_value = Box::new(123); // `heap_value` points to an integer on the heap

    println!("Heap value: {}", heap_value);

    // Create an `Rc` (Reference Counted) to manage heap-allocated data with shared ownership
    let rc_value = Rc::new(456);
    let rc_value_clone = Rc::clone(&rc_value); // Clone the `Rc`, increasing the reference count

    println!("Rc value: {}", rc_value);
    println!("Rc cloned value: {}", rc_value_clone);
}

fn main() {
    // Demonstrate stack memory usage
    println!("Demonstrating stack memory:");
    stack_memory_example();

    // Demonstrate heap memory usage
    println!("\nDemonstrating heap memory:");
    heap_memory_example();
}*/

/*use std::io;

fn main() {
    // Kullanıcıdan metin ve kaydırma anahtarı girmesini iste
    let mut text = String::new();
    let mut shift_key = String::new();

    // Kullanıcıdan metin girmesini iste
    println!("Şifrelemek için bir metin girin:");
    io::stdin().read_line(&mut text)
        .expect("Girdi okuma hatası");

    // Kullanıcıdan kaydırma anahtarı girmesini iste
    println!("Kaydırma anahtarını girin:");
    io::stdin().read_line(&mut shift_key)
        .expect("Girdi okuma hatası");

    // Kaydırma anahtarını tam sayıya dönüştür
    let shift: i32 = match shift_key.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçerli bir sayı girilmedi.");
            return;
        }
    };

    // Metni şifrele
    let encrypted_text = caesar_cipher(&text.trim(), shift);

    // Şifrelenmiş metni ekrana yazdır
    println!("Şifrelenmiş metin: {}", encrypted_text);
}

// Caesar şifreleme fonksiyonu
fn caesar_cipher(text: &str, shift: i32) -> String {
    let mut result = String::new();

    // Kaydırma anahtarını 26 ile mod alarak 0-25 aralığına getir
    let shift = shift % 26;

    // Her karakteri işleyerek şifrele
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            // Karakteri büyük harfse
            if c.is_ascii_uppercase() {
                let base = 'A' as u8;
                let new_char = (((c as u8 - base + shift as u8) % 26) + base) as char;
                result.push(new_char);
            }
            // Karakteri küçük harfse
            else if c.is_ascii_lowercase() {
                let base = 'a' as u8;
                let new_char = (((c as u8 - base + shift as u8) % 26) + base) as char;
                result.push(new_char);
            }
        } else {
            // Harf olmayan karakterleri değiştirmeden ekle
            result.push(c);
        }
    }

    result
}*/

/*use std::io;

struct MinStack {
    stack: Vec<i32>,       // Asıl yığın, verilerin saklandığı yer
    min_stack: Vec<i32>,   // Minimum değerleri saklamak için kullanılan yardımcı yığın
}

impl MinStack {
    // Yeni bir MinStack oluşturur
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),    // Boş bir yığın oluştur
            min_stack: Vec::new(), // Boş bir min_stack oluştur
        }
    }

    // Bir elemanı yığına ekler ve min_stack'i günceller
    fn push(&mut self, value: i32) {
        self.stack.push(value); // Elemanı asıl yığına ekler
        let min_val = self.min_stack.last().cloned().unwrap_or(i32::MAX);
        self.min_stack.push(min_val.min(value)); // Yeni minimum değeri hesaplar ve min_stack'e ekler
    }

    // Yığının üstündeki elemanı çıkarır ve min_stack'i günceller
    fn pop(&mut self) {
        if self.stack.pop().is_some() {
            self.min_stack.pop(); // Min_stack'teki ilgili minimum değeri çıkarır
        }
    }

    // Yığının üstündeki elemanı döner
    fn top(&self) -> Option<i32> {
        self.stack.last().cloned() // Yığının üstündeki elemanı döner
    }

    // Yığındaki minimum elemanı döner
    fn min(&self) -> Option<i32> {
        self.min_stack.last().cloned() // Min_stack'teki en üst minimum değeri döner
    }
}

fn main() {
    let mut min_stack = MinStack::new(); // Yeni bir MinStack oluştur
    let mut input = String::new(); // Kullanıcıdan alınacak girdiyi saklamak için bir değişken

    loop {
        println!("Choose an option:");
        println!("1: Push element");
        println!("2: Pop element");
        println!("3: Get top element");
        println!("4: Get minimum element");
        println!("5: Exit");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                input.clear();
                println!("Enter the value to push:");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let value: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a valid integer.");
                        continue;
                    }
                };
                min_stack.push(value); // Yığına değeri ekler
            },
            2 => {
                min_stack.pop(); // Üstteki elemanı çıkarır
                println!("Element popped.");
            },
            3 => {
                match min_stack.top() {
                    Some(top) => println!("Top element: {}", top),
                    None => println!("Stack is empty."),
                }
            },
            4 => {
                match min_stack.min() {
                    Some(min_val) => println!("Minimum element: {}", min_val),
                    None => println!("Stack is empty."),
                }
            },
            5 => {
                println!("Exiting...");
                break; // Döngüden çıkar ve programı bitir
            },
            _ => println!("Invalid choice, please enter a number between 1 and 5."),
        }
    }
}*/



use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Choose a module to run (1-5) or 0 to exit:");

        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => ders1::run(),
            2 => ders2::run(),
            3 => ders3::run(),
            4 => ders4::run(),
            5 => ders5::run(),
            0 => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please enter a number between 1 and 5, or 0 to exit."),
        }
    }
}

