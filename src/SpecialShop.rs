#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SpecialShopSheet {
exd: EXD,
exh: EXH,
}
impl SpecialShopSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("SpecialShop")?;let exd = game_data.read_excel_sheet("SpecialShop", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<SpecialShopRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SpecialShopRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<SpecialShopRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<SpecialShopRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct SpecialShopRow {
columns: Vec<ColumnData>,
}
impl SpecialShopRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> [&ColumnData; 60] {
[&self.columns[1],&self.columns[2],&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],&self.columns[23],&self.columns[24],&self.columns[25],&self.columns[26],&self.columns[27],&self.columns[28],&self.columns[29],&self.columns[30],&self.columns[31],&self.columns[32],&self.columns[33],&self.columns[34],&self.columns[35],&self.columns[36],&self.columns[37],&self.columns[38],&self.columns[39],&self.columns[40],&self.columns[41],&self.columns[42],&self.columns[43],&self.columns[44],&self.columns[45],&self.columns[46],&self.columns[47],&self.columns[48],&self.columns[49],&self.columns[50],&self.columns[51],&self.columns[52],&self.columns[53],&self.columns[54],&self.columns[55],&self.columns[56],&self.columns[57],&self.columns[58],&self.columns[59],&self.columns[60],]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[61]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[62]
}
pub fn RequiredContentFinderCondition(&self) -> &ColumnData {
&self.columns[63]
}
pub fn CompleteText(&self) -> &ColumnData {
&self.columns[64]
}
pub fn NotCompleteText(&self) -> &ColumnData {
&self.columns[65]
}
pub fn RequiredFestival(&self) -> &ColumnData {
&self.columns[66]
}
pub fn RequiredFestivalPhase(&self) -> &ColumnData {
&self.columns[67]
}
pub fn UseCurrencyType(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[69]
}
/// If this is true, then the CFC needs to be completed; If this is false, then the CFC just needs to be unlocked
pub fn RequiredContentFinderConditionComplete(&self) -> &ColumnData {
&self.columns[70]
}
}
