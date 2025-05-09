#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GCScripShopItemSheet {
exd: EXD,
exh: EXH,
}
impl GCScripShopItemSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("GCScripShopItem")?;let exd = game_data.read_excel_sheet("GCScripShopItem", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
pub fn get_row(&self, id: u32) -> Option<GCScripShopItemRow> {
let Some(ExcelRowKind::SingleRow(row)) = &self.exd.get_row(id) else { return None; };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GCScripShopItemRow { columns })
}
}
pub struct GCScripShopItemRow {
columns: Vec<ColumnData>,
}
impl GCScripShopItemRow {
pub fn CostGCSeals(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn RequiredGrandCompanyRank(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[3]
}
}
