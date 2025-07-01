#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct TripleTriadSheet {
exd: EXD,
exh: EXH,
}
impl TripleTriadSheet {
pub fn read_from(game_data: &mut GameData, language: Language) -> Option<Self> {
let exh = game_data.read_excel_sheet_header("TripleTriad")?;let exd = game_data.read_excel_sheet("TripleTriad", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<TripleTriadRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(TripleTriadRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<TripleTriadRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<TripleTriadRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct TripleTriadRow {
columns: Vec<ColumnData>,
}
impl TripleTriadRow {
pub fn ItemPossibleReward(&self) -> [&ColumnData; 4] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],]
}
pub fn PreviousQuest(&self) -> [&ColumnData; 3] {
[&self.columns[4],&self.columns[5],&self.columns[6],]
}
pub fn DefaultTalkChallenge(&self) -> &ColumnData {
&self.columns[7]
}
pub fn DefaultTalkUnavailable(&self) -> &ColumnData {
&self.columns[8]
}
pub fn DefaultTalkNPCWin(&self) -> &ColumnData {
&self.columns[9]
}
pub fn DefaultTalkDraw(&self) -> &ColumnData {
&self.columns[10]
}
pub fn DefaultTalkPCWin(&self) -> &ColumnData {
&self.columns[11]
}
pub fn TripleTriadCardFixed(&self) -> [&ColumnData; 5] {
[&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],]
}
pub fn TripleTriadCardVariable(&self) -> [&ColumnData; 5] {
[&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],]
}
pub fn Fee(&self) -> &ColumnData {
&self.columns[22]
}
pub fn StartTime(&self) -> &ColumnData {
&self.columns[23]
}
pub fn EndTime(&self) -> &ColumnData {
&self.columns[24]
}
pub fn TripleTriadRule(&self) -> [&ColumnData; 2] {
[&self.columns[25],&self.columns[26],]
}
pub fn PreviousQuestJoin(&self) -> &ColumnData {
&self.columns[27]
}
pub fn UsesRegionalRules(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[29]
}
}
