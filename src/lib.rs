#[macro_export]

macro_rules! kukumba {

    (
        $ErrorType:ty,
        $(
            #[$unit:ident]
            $( $($op:ident $desc:literal)+ { $($stmt:stmt)* } )*
        )*
    ) => {
        $(#[test] fn $unit () -> Result<(), $ErrorType> {
            $(
                $(print!("\n  {} {}", stringify!($op), $desc);)+
                print!(" ");
                $($stmt;)*
            )*
            Ok(())
        })*
    };

}
