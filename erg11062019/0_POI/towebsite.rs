
/*

Hannes Karl: IMG to WEBSITE build, 2019

call: 
*/

use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {

    let mut htmloutput = String::from("<!DOCTYPE html><html lang=\"de\"><head></head><body>");

    for entry in fs::read_dir( "." )? {
        let dir = entry?;
        let t =  dir.file_name( );
        let tt = t.into_string( );
        let the_result = tt.unwrap();
        htmloutput.push_str( "<div>Name: " );
        htmloutput.push_str( &the_result );
        htmloutput.push_str( "</div>" );
        htmloutput.push_str( "<img style=\"width:100%;\" src=\"" );
        htmloutput.push_str( &the_result );
        htmloutput.push_str( "\"/><br>\n" );
        println!( "{:?}", &the_result );
    }
    htmloutput.push_str( "</body></html>" );
    let mut file = File::create( "index.html" )?;
    file.write_all( htmloutput.as_bytes( ) );
    
    Ok(())
}
