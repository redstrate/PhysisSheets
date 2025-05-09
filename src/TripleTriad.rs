#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct TripleTriad {
exd: EXD,
exh: EXH,
}
impl TripleTriad {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("TripleTriad").unwrap();let exd = game_data.read_excel_sheet("TripleTriad", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> TripleTriadRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
TripleTriadRow { columns }
}
}
pub struct TripleTriadRow {
columns: Vec<ColumnData>,
}
impl TripleTriadRow {
pub fn ItemPossibleReward(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PreviousQuest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn DefaultTalkChallenge(&self) -> &ColumnData {
&self.columns[2]
}
pub fn DefaultTalkUnavailable(&self) -> &ColumnData {
&self.columns[3]
}
pub fn DefaultTalkNPCWin(&self) -> &ColumnData {
&self.columns[4]
}
pub fn DefaultTalkDraw(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DefaultTalkPCWin(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TripleTriadCardFixed(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TripleTriadCardVariable(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Fee(&self) -> &ColumnData {
&self.columns[9]
}
pub fn StartTime(&self) -> &ColumnData {
&self.columns[10]
}
pub fn EndTime(&self) -> &ColumnData {
&self.columns[11]
}
pub fn TripleTriadRule(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PreviousQuestJoin(&self) -> &ColumnData {
&self.columns[13]
}
pub fn UsesRegionalRules(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[15]
}
}
