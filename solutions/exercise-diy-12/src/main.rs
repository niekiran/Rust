/*
Exercise-diy-12
Partial code of the inventory management application
The code currently uses panic!() for error handling, you have to refactor the code to
replace all instances of panic! with proper error handling using the Result enum
and a custom error type InventoryError.
Ensure that each method in the Inventory struct returns a Result type,
providing meaningful error messages and handling errors gracefully.
 */

 use std::collections::HashMap;

 #[derive(Debug)]
 enum InventoryError {
     ItemNotFound,
     DuplicateItem(Item),
 }
 
 #[derive(Debug, Clone)]
 struct Item {
     id: u32,
     name: String,
     quantity: u32,
     price: f64,
 }
 
 struct Inventory {
     items: HashMap<u32, Item>,
     next_item_id: u32,
 }
 
 impl Inventory {
     fn new() -> Self {
         Inventory {
             items: HashMap::new(),
             next_item_id: 1,
         }
     }
 
     fn add_item(&mut self, name: String, quantity: u32, price: f64) -> Result<(), InventoryError> {
         if let Some(item) = self.items.values().find(|item| item.name == name) {
             return Err(InventoryError::DuplicateItem(item.clone()));
         }
 
         let id = self.next_item_id;
         self.next_item_id += 1;
 
         self.items.insert(
             id,
             Item {
                 id,
                 name,
                 quantity,
                 price,
             },
         );
 
         Ok(())
     }
 
     fn update_item(
         &mut self,
         id: u32,
         name: Option<String>,
         quantity: Option<u32>,
         price: Option<f64>,
     ) -> Result<(), InventoryError> {
         match self.items.get_mut(&id) {
             Some(item) => {
                 if let Some(name) = name {
                     item.name = name;
                 }
                 if let Some(quantity) = quantity {
                     item.quantity = quantity;
                 }
                 if let Some(price) = price {
                     item.price = price;
                 }
                 Ok(())
             }
             None => Err(InventoryError::ItemNotFound),
         }
     }
 
     fn delete_item(&mut self, id: u32) -> Result<(), InventoryError> {
         if self.items.remove(&id).is_some() {
             Ok(())
         } else {
             Err(InventoryError::ItemNotFound)
         }
     }
 
     fn list_items(&self) -> Result<Vec<&Item>, InventoryError> {
         Ok(self.items.values().collect())
     }
 
     fn find_item(&self, name: &str) -> Result<&Item, InventoryError> {
         self.items
             .values()
             .find(|item| item.name == name)
             .ok_or(InventoryError::ItemNotFound)
     }
 }
 
 fn main() {
     let mut inventory = Inventory::new();
 
     // Example usage
     match inventory.add_item("Laptop".to_string(), 10, 999.99) {
         Ok(_) => println!("Item added!"),
         Err(e) => println!("Error: {:?}", e),
     }
 
     match inventory.add_item("Smartphone".to_string(), 20, 499.99) {
         Ok(_) => println!("Item added!"),
         Err(e) => println!("Error: {:?}", e),
     }
 
     println!("/////Inventory/////");
     for item in inventory.list_items().unwrap() {
         println!("{:?}", item);
     }
     println!("/////End/////");
 
     inventory
         .update_item(1, Some("Gaming Laptop".to_string()), None, Some(1299.99))
         .unwrap();
     inventory.delete_item(2).unwrap();
 
     match inventory.find_item("Gaming Laptop") {
         Ok(item) => println!("Found item: {:?}", item),
         Err(e) => println!("Error: {:?}", e),
     }
 
     match inventory.find_item("Business Laptop") {
         Ok(item) => println!("Found item: {:?}", item),
         Err(e) => println!("Error: {:?}", e),
     }
 
     match inventory.add_item("Gaming Laptop".to_string(), 10, 879.99) {
         Ok(_) => println!("Item added!"),
         Err(e) => println!("Error: {:?}", e),
     }
 }
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_add_item_should_be_ok() {
         let mut inventory = Inventory::new();
         assert!(inventory
             .add_item("Test Item".to_string(), 10, 9.99)
             .is_ok());
     }
 
     //Write more test cases
 }
 