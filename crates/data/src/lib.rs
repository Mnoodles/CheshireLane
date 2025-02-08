mod schemas;

enum DataType {
    ShareCfg,
    ShareCfgData,
}

macro_rules! data_loader {
    ($([$data_ty:ident::$ty:ident] schemas::$schema:ident::$name:ident;)*) => {
        $(paste::paste! {
            pub mod [<$name:snake _data>] {
                pub const FILE_NAME: &str = concat!(stringify!([<$name:snake>]), ".json");

                pub static DATA: ::std::sync::OnceLock<crate::schemas::$schema::$name> =
                    ::std::sync::OnceLock::new();

                pub fn load(path: &str) -> ::std::io::Result<()> {
                    let data = ::std::fs::read_to_string(path)?;
                    let data = serde_json::from_str::<crate::schemas::$schema::$name>(&data)?;

                    let _ = DATA.set(data);
                    common::logging::info!("Loaded {} data successfully.", stringify!($name));
                    Ok(())
                }
            }
        })*

        pub fn load_all() -> ::std::io::Result<()> {
            $(paste::paste! {
                let path = match $data_ty::$ty {
                    $data_ty::ShareCfg => format!("{}/{}",
                        config::CONFIG.resource.share_cfg_dir,
                        [<$name:snake _data>]::FILE_NAME),
                    $data_ty::ShareCfgData => format!("{}/{}",
                        config::CONFIG.resource.share_cfg_data_dir,
                        [<$name:snake _data>]::FILE_NAME),
                };

                [<$name:snake _data>]::load(&path)?;
            })*

            Ok(())
        }
    };
}

data_loader!(
    [DataType::ShareCfgData] schemas::ship_data_template::ShipDataTemplate;
    [DataType::ShareCfg] schemas::ship_skin_template::ShipSkinTemplate;
);
