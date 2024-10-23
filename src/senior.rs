use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

struct Store {
    products: HashMap<String, Product>,
}

impl Store {
    fn new() -> Store {
        Store {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, price: f64, quantity: u32) -> Result<(), &'static str> {
        // Реализуйте добавление товара с проверкой корректности данных
        if self.products.contains_key(&name){
            return Err("This product is already exists");
        }

        if price <= 0.0{
            return Err("Price can only be a positive number");
        }

        if quantity <= 0 {
            return Err("Quantity of product cannot be negative or equal zero");
        }

        let product = Product{name: name.clone(), price, quantity};
        self.products.insert(name, product);
        return Ok(());
    }

    fn update_product(&mut self, name: &str, new_price: Option<f64>, new_quantity: Option<u32>) -> Result<(), &'static str> {
        // Реализуйте обновление данных о товаре с проверкой наличия товара
        if let Some(product) = self.products.get_mut(name){
            if let Some(price) = new_price{
                if price <= 0.0{
                    return Err("Price can only be a positive number");
                }

                product.price = price;
            }

            if let Some(quantity) = new_quantity{
                if quantity <= 0{
                    return Err("Quantity of product cannot be negative or equal zero");
                }

                product.quantity = quantity;
            }

            return Ok(());
        } 
        return Err("Product does not exist");
    }

    fn find_product(&self, name: &str) -> Option<&Product> {
        // Реализуйте поиск товара по названию
        return self.products.get(name);
    }

    fn remove_product(&mut self, name: &str) -> Result<(), &'static str> {
        // Реализуйте удаление товара с проверкой наличия товара
        if let Some(p) = self.find_product(name){
            self.products.remove(name);
            return Ok(())
        }

        return Err("Product does not exist")
    }
}



fn main(){
    let mut store = Store::new();
    
    // Попытка добавить товар
    match store.add_product("Laptop".to_string(), 1000.0, 10) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }

    // Проверка добавления
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }

    // Попытка добавить уже существующий товар
    match store.add_product("Laptop".to_string(), 2000.0, 25) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }
    
    // Обновление цены товара
    match store.update_product("Laptop", Some(900.0), None) {
        Ok(_) => println!("Product updated successfully"),
        Err(e) => println!("Error updating product: {}", e),
    }

    // Проверка обновления
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }
    
    // Удаление товара
    match store.remove_product("Laptop") {
        Ok(_) => println!("Product removed successfully"),
        Err(e) => println!("Error removing product: {}", e),
    }

    // Проверка удаления
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }
}