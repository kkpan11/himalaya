use clap::Parser;
use color_eyre::Result;
use email::{backend::feature::BackendFeatureSource, folder::list::ListFolders};
use tracing::info;

use crate::{
    account::arg::name::AccountNameFlag,
    backend::Backend,
    config::Config,
    folder::{Folders, FoldersTable},
    printer::Printer,
};

/// List all folders.
///
/// This command allows you to list all exsting folders.
#[derive(Debug, Parser)]
pub struct FolderListCommand {
    #[command(flatten)]
    pub account: AccountNameFlag,

    /// The maximum width the table should not exceed.
    ///
    /// This argument will force the table not to exceed the given
    /// width in pixels. Columns may shrink with ellipsis in order to
    /// fit the width.
    #[arg(long, short = 'w', name = "table_max_width", value_name = "PIXELS")]
    pub table_max_width: Option<u16>,
}

impl FolderListCommand {
    pub async fn execute(self, printer: &mut impl Printer, config: &Config) -> Result<()> {
        info!("executing list folders command");

        let (toml_account_config, account_config) = config
            .clone()
            .into_account_configs(self.account.name.as_deref())?;

        let list_folders_kind = toml_account_config.list_folders_kind();

        let backend = Backend::new(
            toml_account_config.clone(),
            account_config.clone(),
            list_folders_kind,
            |builder| builder.set_list_folders(BackendFeatureSource::Context),
        )
        .await?;

        let folders = Folders::from(backend.list_folders().await?);
        let table = FoldersTable::from(folders)
            .with_some_width(self.table_max_width)
            .with_some_preset(toml_account_config.folder_list_table_preset())
            .with_some_name_color(toml_account_config.folder_list_table_name_color())
            .with_some_desc_color(toml_account_config.folder_list_table_desc_color());

        printer.out(table)?;
        Ok(())
    }
}
