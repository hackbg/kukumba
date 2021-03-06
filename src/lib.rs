#[macro_export]

macro_rules! kukumba {

    ($(
        #[$unit:ident]
        $( $($op:ident $desc:literal)+ { $($stmt:stmt)* } )*
    )*) => {
        $(#[test] fn $unit () {
            $(
                println!("");
                $(println!("{} {}", stringify!($op), $desc);)+
                $($stmt;)*
            )*
        })*
    };

}
