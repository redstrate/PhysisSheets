#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct Recipe {
exd: EXD,
exh: EXH,
}
impl Recipe {
pub fn read_from(game_data: &mut GameData, language: Language) -> Self {
let exh = game_data.read_excel_sheet_header("Recipe").unwrap();let exd = game_data.read_excel_sheet("Recipe", &exh, language, 0).unwrap();Self {
exh,
exd,
}
}
pub fn get_row(&self, id: u32) -> RecipeRow {let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!("Expected a single row!"); };
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
RecipeRow { columns }
}
}
pub struct RecipeRow {
columns: Vec<ColumnData>,
}
impl RecipeRow {
pub fn RequiredQuality(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Quest(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Number(&self) -> &ColumnData {
&self.columns[2]
}
pub fn CraftType(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ItemResult(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Ingredient(&self) -> &ColumnData {
&self.columns[5]
}
pub fn StatusRequired(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ItemRequired(&self) -> &ColumnData {
&self.columns[7]
}
pub fn RecipeLevelTable(&self) -> &ColumnData {
&self.columns[8]
}
pub fn MaxAdjustableJobLevel(&self) -> &ColumnData {
&self.columns[9]
}
pub fn RecipeNotebookList(&self) -> &ColumnData {
&self.columns[10]
}
pub fn DisplayPriority(&self) -> &ColumnData {
&self.columns[11]
}
pub fn DifficultyFactor(&self) -> &ColumnData {
&self.columns[12]
}
pub fn QualityFactor(&self) -> &ColumnData {
&self.columns[13]
}
pub fn DurabilityFactor(&self) -> &ColumnData {
&self.columns[14]
}
pub fn RequiredCraftsmanship(&self) -> &ColumnData {
&self.columns[15]
}
pub fn RequiredControl(&self) -> &ColumnData {
&self.columns[16]
}
pub fn QuickSynthCraftsmanship(&self) -> &ColumnData {
&self.columns[17]
}
pub fn QuickSynthControl(&self) -> &ColumnData {
&self.columns[18]
}
pub fn SecretRecipeBook(&self) -> &ColumnData {
&self.columns[19]
}
pub fn CollectableMetadata(&self) -> &ColumnData {
&self.columns[20]
}
pub fn PatchNumber(&self) -> &ColumnData {
&self.columns[21]
}
pub fn AmountResult(&self) -> &ColumnData {
&self.columns[22]
}
pub fn AmountIngredient(&self) -> &ColumnData {
&self.columns[23]
}
pub fn MaterialQualityFactor(&self) -> &ColumnData {
&self.columns[24]
}
pub fn CollectableMetadataKey(&self) -> &ColumnData {
&self.columns[25]
}
pub fn IsSecondary(&self) -> &ColumnData {
&self.columns[26]
}
pub fn CanQuickSynth(&self) -> &ColumnData {
&self.columns[27]
}
pub fn CanHq(&self) -> &ColumnData {
&self.columns[28]
}
pub fn ExpRewarded(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[30]
}
pub fn IsSpecializationRequired(&self) -> &ColumnData {
&self.columns[31]
}
pub fn IsExpert(&self) -> &ColumnData {
&self.columns[32]
}
}
