
/*

Hannes Karl: IMG to WEBSITE build, 2019

call: 
*/

use std::fs;
use std::fs::File;
use std::io::Write;



fn main() -> std::io::Result<()> {
    let stich1 = [
    "001Test",
"002Test",
"003Test",
"004Test",
"005Test",
"006Test",
"007Test",
"008Test",
"009Test",
"010Test",
"011Test"
,"012Test"
,"013Test"
,"014Test"
,"015Test"
,"016Test"
,"017Test"
,"018Test"
,"019Test"
,"020Test"
,"021Test"
,"022Test"
,"023Test"
,"024Test"
,"025Test"
,"026Test"
,"027Test"
,"028Test"
,"029Test"
,"030Test"
,"031Test"
//,"032Test"
//,"033Test"
//,"034Test"
,"035Test"
,"036Test"
,"037Test"
,"038Test"
,"039Test"
,"040Test"
,"041Test"
,"042Test"
,"043Test"
,"044Test"
,"045Test"
,"046Test"
,"047Test"
,"048Test"
,"049Test"
,"050Test"
,"051Test"
,"052Test"
,"053Test"
,"054Test"
,"055Test"
,"056Test"
,"057Test"
,"058Test"
,"059Test"
,"060Test"
,"061Test"
,"062Test"
,"063Test"
,"064Test"
,"065Test"
,"066Test"
,"067Test"
,"068Test"
,"069Test"
,"070Test"
,"071Test"
,"072Test"
,"073Test"
,"074Test"
,"075Test"
,"076Test"
,"077Test"
,"078Test"
,"079Test"
,"080Test"
,"081Test"
,"082Test"
,"086Test"
,"087Test"
,"088Test"
,"089Test"
,"090Test"
,"091Test"
,"092Test"
,"093Test"
,"094Test"
,"095Test"
,"096Test"
,"097Test"
,"098Test"
,"099Test"
,"100Test"
,"101Test"
,"102Test"
,"104Test"
,"105Test"
,"106Test"
,"107Test"
,"108Test"
,"109Test"
,"110Test"
,"111Test"
,"112Test"
,"113Test"
,"114Test"
,"115Test"
,"116Test"
,"117Test"
,"118Test"
,"119Test"
,"120Test"
,"121Test"
,"122Test"
,"123Test"
,"125Test"
,"126Test"
,"127Test"
,"128Test"
    ];
    let stich2 = [
            "E3-144r_002k",
            "E3-144r_003k",
            "E3-144r_004k",
            "E3-144r_005k",
            "E3-144r_006k",
"E3-144r_007k",
"E3-144r_008k",
"E3-144r_009k",
"E3-144rk",
"E3-144v_002k",
"E3-144v_003k",
"E3-144v_004k",
"E3-144v_005k",
"E3-144v_006k",
"E3-144v_007k",
"E3-144v_008k",
"E3-144v_009k",
"E3-144vk",
"aepinus_bekentnis_1548_0006",
"aepinus_bekentnis_1548_0007",
"alberti_pictura_1540_0007",
"alberti_pictura_1540_0008",
"alberti_pictura_1540_0009",
"aventinus_grammatica_1515_0007",
"aventinus_grammatica_1515_0008",
"basilius_legendi_1515_0007",
"basilius_legendi_1515_0008",
"boeckel_oratio_1589_00021",
"boeschenstain_gedicht_1520_0002",
"brenz_abentmal_1550_0043",
"brenz_abentmal_1550_0158",
"dorn_uppedat_1507_00017",
"dorn_uppedat_1507_00032",
"feyge_epistole_1500_00007",
"feyge_epistole_1500_00023",
"heyden_paedono_1548_0007",
"heyden_paedono_1548_0013",
"karlstadt_sermon_1523_0006",
"karlstadt_sermon_1523_0020",
"kistler_kraeuter_1500_0007",
"kistler_kraeuter_1500_0021",
"lucius_epithalamia_1585_00006",
"lucius_epithalamia_1585_00014",
"luther_babstum_1526_0010",
"luther_babstum_1526_0011",
"nn_historia_1500_0007",
"nn_historia_1500_0009",
"oesterreicher_sachsen_1548_0007",
"oesterreicher_sachsen_1548_0011",
"osiander_predigt_1553_0007",
"osiander_predigt_1553_0008",
"petrarca_psalmi_1506_0010",
"petrarca_psalmi_1506_0012",
"pinder_epiphanie_1506_0009",
"pinder_epiphanie_1506_0010",
"pistoris_regiment_1506_0009",
"pistoris_regiment_1506_0010",
"rhegius_artzney_1529_0007",
"rhegius_artzney_1529_0014",
"sachs_drey_1553_0010",
"sachs_drey_1553_0011",
"trota_mordtbrenner_1540_0011",
"trota_mordtbrenner_1540_0013",
"vespucci_insule_1506_0009",
"vespucci_insule_1506_0019",
"witzstat_buchszbaum_1540_0018",
"witzstat_buchszbaum_1540_0021",
"Rep_M_03_0010a",
"Rep_M_03_0010b",
"Rep_M_03_0011a",
"Rep_M_03_0011b",
"Rep_M_03_0012a",
"Rep_M_03_0012b",
"Rep_M_03_0013a",
"Rep_M_03_0013b",
"Rep_M_03_0014a",
"Rep_M_03_0014b",
"Rep_M_03_0015a",
"Rep_M_03_0015b",
"Rep_M_03_0016a",
"Rep_M_03_0016b",
"Rep_M_03_0017a",
"Rep_M_03_0017b",
"Rep_M_03_0018a",
"Rep_M_03_0018b",
"AldusManutius10",
"AldusManutius11",
"AldusManutius12",
"AldusManutius13",
"AldusManutius14",
"AldusManutius15",
"AldusManutius16",
"AldusManutius18",
"AldusManutius6",
"AldusManutius7",
"AldusManutius8",
"AldusManutius9",
"DionysiusHalicarnassensis170",
"DionysiusHalicarnassensis171",
"DionysiusHalicarnassensis172",
"DionysiusHalicarnassensis173",
"DionysiusHalicarnassensis174",
"DionysiusHalicarnassensis175",
"DionysiusHalicarnassensis176",
"DionysiusHalicarnassensis177",
"DionysiusHalicarnassensis178",
"DionysiusHalicarnassensis179",
"DionysiusHalicarnassensis180",
"GiorgioInteriano10",
"GiorgioInteriano11",
"GiorgioInteriano12",
"GiorgioInteriano13",
"GiorgioInteriano14",
"GiorgioInteriano15",
"GiorgioInteriano16",
"GiorgioInteriano17",
"GiorgioInteriano18",
"GiorgioInteriano19",
"GiorgioInteriano22",
"Pindarus350",
"Pindarus351",
"Pindarus352",
"Pindarus353",
"Pindarus354",
"Pindarus355",
"Pindarus356",
"Pindarus357",
"Pindarus358",
"Pindarus359",
"Pindarus360",
"SimpliciusCilicius310",
"SimpliciusCilicius311",
"SimpliciusCilicius312",
"SimpliciusCilicius313",
"SimpliciusCilicius314",
"SimpliciusCilicius315",
"SimpliciusCilicius316",
"SimpliciusCilicius317",
"SimpliciusCilicius318",
"SimpliciusCilicius319",
"SimpliciusCilicius320",
"TheodorusGaza100",
"TheodorusGaza101",
"TheodorusGaza102",
"TheodorusGaza103",
"TheodorusGaza104",
"TheodorusGaza105",
"TheodorusGaza106",
"TheodorusGaza141",
"TheodorusGaza19",
"TheodorusGaza7",
"EuripidesBE_0750_0018",
"EuripidesBE_0750_0019",
"EuripidesBE_0750_0020",
"EuripidesBE_0750_0021",
"EuripidesBE_0750_0022",
"EuripidesBE_0750_0023",
"EuripidesBE_0750_0024",
"EuripidesBE_0750_0025",
"EuripidesBE_0750_0026",
"EuripidesBE_0750_0027",
"EuripidesBE_0750_0028",
"hypnerotomachiapoliphilii17r",
"hypnerotomachiapoliphilii17v",
"hypnerotomachiapoliphilii18r",
"hypnerotomachiapoliphilii18v",
"hypnerotomachiapoliphilii19r",
"hypnerotomachiapoliphilii19v",
"hypnerotomachiapoliphilii20r",
"hypnerotomachiapoliphilii20v",
"hypnerotomachiapoliphilii21r",
"hypnerotomachiapoliphilii21v",
"hypnerotomachiapoliphilii22r",
"hypnerotomachiapoliphilii22v",
"hypnerotomachiapoliphilii23r",
"hypnerotomachiapoliphilii23v",
"hypnerotomachiapoliphilii24r",
"hypnerotomachiapoliphilii24v",
"comoediaenovem9r",
"comoediaenovem9v",
"comoediaenovem10r",
"comoediaenovem10v",
"comoediaenovem11r",
"comoediaenovem11v",
"comoediaenovem12r",
"comoediaenovem12v",
"comoediaenovem13r",
"comoediaenovem13v",
"comoediaenovem14r",
"comoediaenovem14v",
"comoediaenovem15r",
"comoediaenovem15v",
"comoediaenovem16r",
"comoediaenovem16v",
"e-codices_csg-0018_072_max", 
"e-codices_csg-0018_081_max",  
"e-codices_csg-0018_157_max",  
"e-codices_csg-0018_158_max",  
"e-codices_csg-0018_162_max", 
"e-codices_csg-0018_165_max",  
"e-codices_csg-0018_169_max",  
"e-codices_csg-0018_183_max",  
"e-codices_csg-0018_184_max",  
"e-codices_csg-0018_185_max",  
"e-codices_fmb-cb-0055_0117r_max",
"e-codices_fmb-cb-0055_0124v_max",
"e-codices_fmb-cb-0055_0125r_max",
"e-codices_fmb-cb-0055_0129v_max",
"e-codices_fmb-cb-0055_0140r_max",
"e-codices_fmb-cb-0055_0140v_max",
"e-codices_fmb-cb-0055_0146r_max",
"e-codices_fmb-cb-0055_0148v_max",
"e-codices_fmb-cb-0055_0158r_max",
"e-codices_fmb-cb-0055_0158v_max"
                ];
    let mut htmloutput = String::from("<!DOCTYPE html><html lang=\"de\"><head></head><body>");
    let mut count = 0;
    //for namepart in stich2.iter() {
    for namepart in stich1.iter() {
        println!("{}", namepart ); 
        let mut naname = String::from( namepart.to_string() );
        naname.push_str( "diffbild.png" );//0_POI
        //naname.push_str( "rand.png" ); //1_
        //naname.push_str( "freigest.png" ); //2_
        //naname.push_str( "boxess.png" ); //3_
        htmloutput.push_str( "<div>Name: " );
        htmloutput.push_str( &naname );
        htmloutput.push_str( "</div>" );
        htmloutput.push_str( "<img style=\"width:100%;\" src=\"" );
        htmloutput.push_str( &naname );
        htmloutput.push_str( "\"/>\n" );
        println!( "{:?}", &naname );

        if count % 20 == 0 && count != 0 {
            htmloutput.push_str( "<a href=\"index" );
            htmloutput.push_str( &(count+20).to_string( ) );
            htmloutput.push_str( ".html\">" );
            htmloutput.push_str( "Weiter</a>" );
            htmloutput.push_str( "</body></html>" );
            let mut nana = String::from("index");
            nana.push_str( &count.to_string( ) );
            nana.push_str( ".html" );
            let mut file = File::create( nana )?;
            file.write_all( htmloutput.as_bytes( ) );
            htmloutput = String::from("<!DOCTYPE html><html lang=\"de\"><head></head><body>");
        }

        count += 1;
    }
    htmloutput.push_str( "</body></html>" );
    let mut nana = String::from("index");
    nana.push_str( &count.to_string( ) );
    nana.push_str( ".html" );
    let mut file = File::create( nana )?;
    file.write_all( htmloutput.as_bytes( ) );
    
    Ok(())
}
