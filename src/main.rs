use postgres::{Client, NoTls, SimpleQueryRow, SimpleQueryMessage};

fn main() {
   
    let client_result = Client::connect("host=remote_postgres_rust user=postgres password=tenTwofourandmanyless_22 dbname=rust_test_db", NoTls);
    {
        let dup_client_result = client_result.as_ref();
        match dup_client_result {
            Ok(..) => { },
            Err(s) => { println!("Client Connect Error {s}"); return; },
        }
    }
    let mut _client = client_result.unwrap();

    let rows_result = Client::simple_query(&mut _client,&"SELECT * FROM books");
    {
        let dup_rows_result = rows_result.as_ref();
        match dup_rows_result {
            Ok(..) => { },
            Err(s) => { println!("Rows Error {s}"); return; },
        }
    }

    let rows = rows_result.unwrap();
    let mut row_idx = 0;

    for curRow in rows 
     {
        match curRow {
            SimpleQueryMessage::Row(the_row) => 
            { 
                row_idx += 1;
                let mut row_image_str = "".to_string(); 
                for col_index in 0 .. the_row.columns().len() {
                    let curr_row_option = the_row.get(col_index);
                    let field_val = the_row.get(col_index).unwrap();
                    row_image_str += ", ";
                    row_image_str +=  field_val;
                }
                println!("Row {}: \"{}\"",row_idx, row_image_str)
                //let mut curr_row : &str = &"".to_string();
                //let c1 = SimpleQueryRow::columns(&the_row).get(0).unwrap();
                //let mut column_name : &str = &"".to_string(); 
                //for cur_column in the_row.columns().iter() {
                //    column_name = cur_column.name();
                //    match curr_row_option {
                //        Some(s) => {  },
                //        None => { println!("Error getting column name"); return; },
                //    }
                //    let x = curr_row;
                //}
            }, 
            SimpleQueryMessage::CommandComplete(the_row_count) => 
            {  
                ()
            },    
            _ => {},       
        }
        //let c1 = SimpleQueryRow::columns(&theRow).get(0).unwrap();
        //println!("Title: ");
    };
    let z = 1;
    _client.close().unwrap();
}
