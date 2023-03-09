use postgres::{Client, NoTls, SimpleQueryMessage};

#[derive(Clone)]
pub struct BooksListReport<'a, 'b> {
    header_printed : bool,
    column_names:&'b Vec<String>, 
    books: &'a Vec<Vec<String>>,
}

fn main() {


    let client_result = Client::connect("host=remote_postgres_rust user=postgres password=tenTwofourandmanyless_22 dbname=rust_test_db", NoTls);
    if !client_result.is_ok() {
        let ref_client_result = client_result.as_ref();
        match ref_client_result {
            Ok(..) => { },
            Err(s) => { println!("Client Connect Error {s}"); return; },
        }
    }
    let mut _client = client_result.unwrap();

    let rows_result = Client::simple_query(&mut _client,&"SELECT * FROM books");
    {
        let ref_rows_result = rows_result.as_ref();
        match ref_rows_result {
            Ok(..) => { },
            Err(s) => { println!("Rows Error {s}"); return; },
        }
    }

    let rows = rows_result.unwrap();
    let mut row_idx = 0;
    let mut rows_count = rows.len() as u64;
    let mut column_names = Vec::new();
    let mut row_column_values : Vec<Vec<String>> = Vec::new();

    for curRow in rows 
     {
        match curRow {
            SimpleQueryMessage::Row(the_row) => 
            { 
                // capture column names on the first row (once)
                if row_idx == 0 {
                    // Skip initial column (primary key id index)
                    for cname in &the_row.columns()[1..] {
                        column_names.push(cname.name().to_string());
                    }
               }
                //row_idx += 1;
                row_column_values.push(Vec::new());
                for col_index in 1 .. the_row.columns().len() {
                    //let curr_row_option = the_row.get(col_index);
                    let field_val = the_row.get(col_index).unwrap().to_string();
                    row_column_values[row_idx].push(field_val);
                }
                row_idx += 1;
            }, 
            SimpleQueryMessage::CommandComplete(the_row_count) => 
            {  
                rows_count = the_row_count;
            },    
            _ => {},       
        }
    };

    {
        let mut books_list = BooksListReport { header_printed:false, column_names:&column_names, books:&row_column_values, };
        print_books(&mut books_list);
    }
    
    // Setup spacing title [50], isbn [20], copywrite_date [20], publisher [40], condition [10], location [14], authors [60]
    _client.close().unwrap();
}

fn print_books(book_info : &mut BooksListReport) {
    const title_column: usize = 0;
    const isbn_column: usize = 1;
    const copyright_date_column : usize = 2;
    const publisher_column : usize = 3;
    const condition_column : usize = 4;
    const location_column : usize = 5;
    const authors_column : usize = 6;
    let mut book_index: usize = 0;
    for current_book in book_info.books {
        // Print this book and its fields
        let mut book_entry_output = " Book Title         => ".to_string();
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[title_column].as_str());
        book_entry_output.push_str("'\n");

        book_entry_output.push_str("     ISBN           => ");
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[isbn_column].as_str());
        book_entry_output.push_str("'\n");

        book_entry_output.push_str("     Copyright date => ");
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[copyright_date_column].as_str());
        book_entry_output.push_str("'\n");

        book_entry_output.push_str("     Publisher      => ");
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[publisher_column].as_str());
        book_entry_output.push_str("'\n");

        book_entry_output.push_str("     Condition      => ");
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[condition_column].as_str());
        book_entry_output.push_str("'\n");

        book_entry_output.push_str("     Location       => ");
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[location_column].as_str());
        book_entry_output.push_str("'\n");

        book_entry_output.push_str("     Authors        => ");
        book_entry_output.push_str("'");
        book_entry_output.push_str( current_book[authors_column].as_str());
        book_entry_output.push_str("'\n");

        println!("{book_entry_output}");
        book_index += 1;
    }
     
}

// Book Title         => "sdfsdfsdfdsfsdfdsf"
//     ISBN           => "sdfsdfsdfsdfsdfdsf"
//     Copyright date => "sdfsdfsdfsdfsdfdsf"
//     Publisher      => "sdfsdfsdfsdfsdfdsf"
//     Condition      => "sdfsdfsdfsdfsdfdsf"
//     Location       => "sdfsdfsdfsdfsdfdsf"
//     Authors        => "sdfsdfsdfsdfsdfdsf"