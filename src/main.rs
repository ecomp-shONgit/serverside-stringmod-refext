

use RUSTYFASTSTRINGTHING::textnorm;
use RUSTYFASTSTRINGTHING::textdecomp;
use RUSTYFASTSTRINGTHING::strdist;
use std::collections::HashSet;
use std::collections::HashMap;

//#[test]
fn alltests(){
    println!("Start USING the rusty fast string thing.");
    let mima = String::from( "hh † „[IX]” ⁙  ἀλλ’ ἑτέραν τινὰ φύσιν ἄπειρον', ἐξ ἧς ἅπαντας γίνεσθαι τοὺς οὐρανοὺς καὶ τοὺς ἐν αὐτοῖς κόσμους· ἐξ ὧν δὲ ἡ γένεσίς ἐστι τοῖς οὖσι, καὶ τὴν φθορὰν εἰς ταῦτα γίνεσθαι κατὰ τὸ χρεών. διδόναι γὰρ αὐτὰ δίκην καὶ τίσιν ἀλλήλοις τῆς ἀδικίας κατὰ τὴν τοῦ χρόνου τάξιν, ποιητικωτέροις οὕτως ὀνόμασιν αὐτὰ λέγων· δῆλον δὲ ὅτι τὴν εἰς ἄλληλα μεταβολὴν τῶν τεττάρων στοιχείων οὗτος θεασάμενος οὐκ ἠξίωσεν ἕν τι τούτων ὑποκείμενον ποιῆσαι, ἀλλά τι ἄλλο παρὰ ταῦτα. οὗτος δὲ οὐκ ἀλλοιουμένου τοῦ στοιχείου τὴν γένεσιν ποιεῖ, ἀλλ’ ἀποκριν-\nομένων τῶν ἐναντίων διὰ τῆς ἀιδίου κινή- σεως· 1 Summá pecúniae, quam dedit in [bla bla bla] aerarium vel plebei Romanae vel dimissis militibus=> denarium sexiens milliens. 2 Opera fecit nova § aedem Martis, Iovis Tonantis et Feretri, Apollinis, díví Iúli, § Quirini, § Minervae, Iunonis Reginae, Iovis Libertatis, Larum, deum Penátium, § Iuventatis, Matris deum, Lupercal, pulvinar ad [11] circum, § cúriam cum chalcidico, forum Augustum, basilicam 35 Iuliam, theatrum Marcelli, § porticus . . . . . . . . . . , nemus trans Tiberím Caesarum. § 3 Refécit Capitolium sacrasque aedes numero octoginta duas, theatrum Pompeí, aquarum rivos, viam Flaminiam.  Ϗ ϗ ϚϛȢȣꙊꙋἀἁἂἃἄἅἆἇἈἉἊἋἌἍἎἏἐἑἒἓἔἕἘἙἚἛἜἝἠἡἢἣἤἥἦἧἨἩἪἫἬἭἮἯἰἱἲἳἴἵἶἷἸἹἺἻἼἽἾἿὀὁὂὃὄὅὈὉὊὋὌὍὐὑὒὓὔὕὖὗὙὛὝὟὠὡὢὣὤὥὦὧὨὩὪὫὬὭὮὯὰάὲέὴήὶίὸόὺύὼώ	ᾀᾁᾂᾃᾄᾅᾆᾇᾈᾉᾊᾋᾌᾍᾎᾏᾐᾑᾒᾓᾔᾕᾖᾗᾘᾙᾚᾛᾜᾝᾞᾟᾠᾡᾢᾣᾤᾥᾦᾧᾨᾩᾪᾫᾬᾭᾮᾯᾰᾱᾲᾳᾴᾶᾷᾸᾹᾺΆᾼ᾽ι᾿῀῁ῂῃῄῆῇῈΈῊΉῌ῍῎῏ῐῑῒΐῖῗῘῙῚΊ῝῞῟ῠῡῢΰῤῥῦῧῨῩῪΎῬ῭΅`ῲῳῴῶῷῸΌῺΏῼ´῾ͰͱͲͳʹ͵Ͷͷͺͻͼͽ;Ϳ΄΅Ά·ΈΉΊΌΎΏΐΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩΪΫάέήίΰαβγδεζηθικλμνξοπρςστυφχψωϊϋόύώϏϐϑϒϓϔϕϖϗϘϙϚϛϜϝϞϟϠϡϢϣϤϥϦϧϨϩϪϫϬϭϮϯϰϱϲϳϴϵ϶ϷϸϹϺϻϼϽϾϿ Αι αι γγ γκ γξ γχ ου Υι υι ἄϋλος αὐλός  τί φῄς; γραφὴν σέ τις, ὡς ἔοικε, γέγραπται οὐ γὰρ ἐκεῖνό γε καταγνώσομαι, ὡς σὺ ἕτερον. δ̣[ὲ κ]αὶ?");
    let atesttext = textnorm::normatextNFD( &mima );

    println!("RUN TESTS STRINGNORM -------------------");
    println!("-- Test: isnumber fn");
    let anum = String::from("aber");
    println!("{} {}", anum, textnorm::isnumber(&anum));
    let bnum = String::from("100");
    println!("{} {}", bnum, textnorm::isnumber(&bnum));
    let cnum = String::from("14c");
    println!("{} {}", cnum, textnorm::isnumber(&cnum));
    let dnum = String::from("lxxxiv");
    println!("{} {}", dnum, textnorm::isnumber(&dnum));
    let eenum = String::from("νδ");
    println!("{} {}", eenum, textnorm::isnumber(&eenum));


    println!("-- Test: disambiguDIAkritika fn {}\n\n", textnorm::disambiguDIAkritika(&atesttext));   
    println!("-- Test: sigmaistgleich fn {}\n\n", textnorm::sigmaistgleich(&atesttext));
    println!("-- Test: deluv fn {}\n\n", textnorm::deluv(&atesttext)); 
    println!("-- Test: sameallspacing fn {}\n\n", textnorm::sameallspacing(&atesttext));
    println!("-- Test: disambiguadashes fn {}\n\n", textnorm::disambiguadashes(&atesttext));   
    println!("-- Test: value textdecomp {}\n\n", textdecomp::avalue);
    let kleinwo = String::from("῾ἑτέραν");
    println!("-- Test: ExtractDiafromBuchst fn {:?}\n\n", textnorm::ExtractDiafromBuchst(&kleinwo));
    println!("-- Test: ExtractDiafromBuchstText fn {:?}\n\n", textnorm::ExtractDiafromBuchstText(&atesttext));
    println!("-- Test: replaceBehauchung fn {:?}\n\n", textnorm::replaceBehauchung(&kleinwo));
    let elu = String::from("ἀλλ’");
    println!("-- Test: Expandelision fn {:?}\n\n", textnorm::Expandelision(&elu));
    println!("-- Test: ExpandelisionText fn {:?}\n\n", textnorm::ExpandelisionText(&atesttext));
    let lilalat = String::from("Aberja");
    println!("-- Test: TranslitLatinGreekLetters fn {}\n\n", textnorm::TranslitLatinGreekLetters( &lilalat ) );
    println!("-- Test: TraslitAncientGreekLatin fn {}\n\n", textnorm::TraslitAncientGreekLatin( &elu ) );
    let spitzeexa = String::from("<aberja>");
    println!("-- Test: spitzeklammernHTML fn {}\n\n", textnorm::spitzeklammernHTML( &spitzeexa ) );
    println!("-- Test: basClean fn {}\n\n", textnorm::basClean( &atesttext ) );

    println!("-- Test: tennsatzei fn {}\n\n", textnorm::tennsatzei( &atesttext ) ); 
    let singlealpha = textnorm::normatextNFD( &"ἄϋλος".to_string() );
    println!("-- Test: AlphaPrivativumCopulativum fn {}\n\n", textnorm::AlphaPrivativumCopulativum( &singlealpha ) ); 
    println!("-- Test: AlphaPrivativumCopulativumText fn {}\n\n", textnorm::AlphaPrivativumCopulativumText( &atesttext ) ); 
    println!("-- Test: ohnediakritW fn {}\n\n", textnorm::ohnediakritW( &atesttext ) ); 
    println!("-- Test: capitali fn {}\n\n", textnorm::capitali( &singlealpha ) ); 
    println!("-- Test: nodiakinword fn {}\n\n", textnorm::nodiakinword( &singlealpha ) );  
    println!("-- Test: delall fn {}\n\n", textnorm::delall( &atesttext ) ); 
    println!("-- Test: delnumbering fn {}\n\n", textnorm::delnumbering( &atesttext ) ); 
    println!("-- Test: deldiak fn {}\n\n", textnorm::deldiak( &atesttext ) ); 
    println!("-- Test: delinterp fn {}\n\n", textnorm::delinterp( &atesttext ) ); 
    println!("-- Test: delunknown fn {}\n\n", textnorm::delunknown( &atesttext ) );  
    println!("-- Test: delumbrbine fn {}\n\n", textnorm::delumbrbine( &atesttext ) ); 
    println!("-- Test: umbrtospace fn {}\n\n", textnorm::umbrtospace( &atesttext ) );  
    let mymarkymark = String::from("<div id=klkl>JJsush </div>");
    println!("-- Test: delmakup fn {}\n\n", textnorm::delmakup( &mymarkymark ) ); 
    println!("-- Test: makuptoleer fn {}\n\n", textnorm::makuptoleer( &mymarkymark ) ); 
    println!("-- Test: delgrkl fn {}\n\n", textnorm::delgrkl( &atesttext ) );  
    println!("-- Test: delklammern fn {}\n\n", textnorm::delklammern( &atesttext ) );  
    println!("-- Test: deledklammern fn {}\n\n", textnorm::deledklammern( &atesttext ) );  
    let little = String::from("ἀλλ’ ἀποκριν-\nομένων τῶν κινή- σεως· 1 Summá alles- [22]\nklar");
    let mywlistl: Vec<&str> = little.split( " " ).collect( );
    println!("-- Test: Trennstricheraus fn {}->{:?}\n\n", little, textnorm::Trennstricheraus( &mywlistl ) ); 
    println!("-- Test: UmbruchzuLeerzeichen fn {}\n\n", textnorm::UmbruchzuLeerzeichen( &atesttext ) ); 
    let mywlist: Vec<&str> = atesttext.split( " " ).collect( ); 
    println!("-- Test: Interpunktiongetrennt fn {:?}\n\n", textnorm::Interpunktiongetrennt( &mywlist ) ); 
    println!("-- Test: iotasubiotoadL fn {:?}\n\n", textnorm::iotasubiotoadL( &mywlist ) );
    println!("-- Test: GRvorbereitungT fn {:?}\n\n", textnorm::GRvorbereitungT( &atesttext ) ); 
    let befco = String::from(" Παῦν̣ι̣ κ");
    let littleunterpunkted = textnorm::normatextNFD( &befco );
    println!("-- Test: delUnterpunkt fn {}->{}\n\n", littleunterpunkted, textnorm::delUnterpunkt( &littleunterpunkted ) ); 

    println!("RUN TESTS STRDIST -------------------");


    let momu = String::from( "hh † „[IX]” ⁙  ἀλλ’ ἑτέραν τινὰ φύσιν ἄπειρον' aber man muss muss sich immer nie nie mit" );
    let mumo = String::from( "mann kann wirklich nicht sagen ⁙  ἀλλ’ ἑτέραν τινὰ sich immer nie nie mit" );
    let k1 = String::from("aibernein");
    let k2 = String::from("aber");
    let stA = textnorm::normatextNFD( &momu );
    let stB = textnorm::normatextNFD( &mumo );

    let vecA: Vec<&str> = stA.split( " " ).collect( );
    let vecB: Vec<&str> = stB.split( " " ).collect( );

    let vecC: Vec<&str> = k1.split( "" ).collect( ); 
    let vecD: Vec<&str> = k2.split( "" ).collect( );

    println!("-- vecA {:?}\n\n", vecA );
    println!("-- vecB {:?}\n\n", vecB ); 

    let setA: HashSet<&str> = strdist::set( &vecA );
    let setB: HashSet<&str> = strdist::set( &vecB );

    println!("-- setA {:?}\n\n", setA );
    println!("-- setB {:?}\n\n", setB );


    println!("--- setA symdiff setB, SetSymDiff fn {:?}\n\n", strdist::SetSymDiff( &setA, &setB )  );
    println!("--- setA diff setB, SetDiff fn {:?}\n\n", strdist::SetDiff( &setA, &setB )  );
    println!("--- setA union setB, SetUnsion fn {:?}\n\n", strdist::SetUnsion( &setA, &setB )  );
    println!("--- setA inters setB, SetIntersection fn {:?}\n\n", strdist::SetIntersection( &setA, &setB )  );
    println!("--- setA symdiff setB len, SetSymDiffLen fn {:?}\n\n", strdist::SetSymDiffLen( &setA, &setB )  );
    println!("--- setA diff setB len, SetDiffLen fn {:?}\n\n", strdist::SetDiffLen( &setA, &setB )  );
    println!("--- setA union setB len, SetUnsionLen fn {:?}\n\n", strdist::SetUnsionLen( &setA, &setB )  );
    println!("--- setA inters setB len, SetIntersectionLen fn {:?}\n\n", strdist::SetIntersectionLen( &setA, &setB )  );

    let wv: Vec<usize> = vec![1, 1, 1, 2];
    let ws: HashMap<String, usize> = HashMap::new();
    println!("--- dist WLEV vecC vecD fn {:?}\n\n", strdist::WLEV( &vecC, &vecD, &wv, &ws )  );
    println!("--- dist LEVDAM vecC vecD fn {:?}\n\n", strdist::LEVDAM( &vecC, &vecD, &wv )  );
    println!("--- dist levenshtein vecC vecD fn {:?}\n\n", strdist::levenshtein( &vecC, &vecD, &wv )  );
    println!("--- dist LCS vecC vecD fn {:?}\n\n", strdist::LCS( &vecC, &vecD )  );
    println!("--- dist LCF vecC vecD fn {:?}\n\n", strdist::LCF( &vecC, &vecD )  );
    println!("--- dist containednessLCS vecC vecD fn {:?}\n\n", strdist::containednessLCS( &vecC, &vecD )  );
    println!("--- dist containednessLCF vecC vecD fn {:?}\n\n", strdist::containednessLCF( &vecC, &vecD )  );
    println!("--- dist LCP vecC vecD fn {:?}\n\n", strdist::LCP( &vecC, &vecD )  );

    println!("--- dist bagdist vecC vecD fn {}\n\n", strdist::bagdist( &vecC, &vecD )  );
    println!("--- dist JA vecC vecD fn {}\n\n", strdist::JA( &vecC, &vecD )  );
    println!("--- dist JAWI vecC vecD fn {}\n\n", strdist::JAWI( &vecC, &vecD )  );
    println!("--- dist baire vecC vecD fn {}\n\n", strdist::baire( &vecC, &vecD )  );
    println!("--- dist generalizedcantor vecC vecD fn {}\n\n", strdist::generalizedcantor( &vecC, &vecD )  );
    println!("--- dist notgeneralizedcantor vecC vecD fn {}\n\n", strdist::notgeneralizedcantor( &vecC, &vecD )  );

    println!("--- dist jaccardMASZzwei vecC vecD fn {}\n\n", strdist::jaccardMASZzwei( &vecC, &vecD )  );
    println!("--- dist jaccardMASZ vecC vecD fn {}\n\n", strdist::jaccardMASZ( &vecC, &vecD )  );

    println!("--- dist cosineMASZ vecC vecD fn {}\n\n", strdist::cosineMASZ( &vecC, &vecD )  );
    println!("--- dist quadradiffMASZ vecC vecD fn {}\n\n", strdist::quadradiffMASZ( &vecC, &vecD )  );
    println!("--- dist diceMASZ vecC vecD fn {}\n\n", strdist::diceMASZ( &vecC, &vecD )  );

    println!("--- dist markingmetric vecC vecD fn {}\n\n", strdist::markingmetric( &vecC, &vecD )  );
    println!("--- dist setdiffmetric vecC vecD fn {}\n\n", strdist::setdiffmetric( &vecC, &vecD )  );
    

    

    
}

fn main() {
    
    alltests();
    println!("Start USING the rusty fast string thing.");
   
    println!("End.");

}
