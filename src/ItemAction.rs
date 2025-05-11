#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ItemActionSheet {
exd: EXD,
exh: EXH,
}
impl ItemActionSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("ItemAction")?;let exd = game_data.read_excel_sheet("ItemAction", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<ItemActionRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ItemActionRow { columns })
}
}
pub struct ItemActionRow {
columns: Vec<ColumnData>,
}
impl ItemActionRow {
pub fn Type(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Data(&self) -> [&ColumnData; 9] {
[&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],]
}
pub fn DataHQ(&self) -> [&ColumnData; 9] {
[&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],]
}
pub fn CondLv(&self) -> &ColumnData {
&self.columns[19]
}
pub fn CondBattle(&self) -> &ColumnData {
&self.columns[20]
}
pub fn CondPVP(&self) -> &ColumnData {
&self.columns[21]
}
pub fn CondPVPOnly(&self) -> &ColumnData {
&self.columns[22]
}
}
