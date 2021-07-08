#[macro_export]

macro_rules! kukumba {

    (
        $(
            #[$unit:ident]
            $( $($op:ident $desc:literal)+ { $($stmt:stmt)* } )*
        )*
    ) => {
        $(#[test] fn $unit <E: Error> () -> Result<(), E> {
            $(
                $(print!("\n  {} {}", stringify!($op), $desc);)+
                print!(": ");
                $($stmt;)*
            )*
            Ok(())
        })*
    };

}
