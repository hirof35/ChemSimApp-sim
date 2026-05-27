use eframe::egui;
use font_kit::font::Font;         // ← 「font-kit」から「font_kit」に修正
use font_kit::properties::Properties; // ← 同上
use font_kit::source::SystemSource;   // ← 同上

// ==========================================
// 1. コア・ドメインモデル（シミュレーション論理）
// ==========================================
#[derive(Debug, Clone)]
pub struct Substance {
    pub name: String,
    pub molar_mass: f64,
    pub mass: f64,
}

impl Substance {
    pub fn moles(&self) -> f64 {
        if self.molar_mass == 0.0 { 0.0 } else { self.mass / self.molar_mass }
    }
}

// ==========================================
// 2. GUI アプリケーションの状態管理
// ==========================================
struct ChemSimApp {
    water: Substance,
    nacl: Substance,
    sugar: Substance,
    volume_liters: f64,
}

impl Default for ChemSimApp {
    fn default() -> Self {
        Self {
            water: Substance { name: "水 (H₂O)".to_string(), molar_mass: 18.02, mass: 1000.0 },
            nacl: Substance { name: "塩化ナトリウム (NaCl)".to_string(), molar_mass: 58.44, mass: 58.44 },
            sugar: Substance { name: "スクロース (C₁₂H₂₂O₁₁)".to_string(), molar_mass: 342.3, mass: 0.0 },
            volume_liters: 1.0,
        }
    }
}

// ==========================================
// 3. GUI レンダリング & インタラクション
// ==========================================
impl eframe::App for ChemSimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🧪 化学溶液濃度シミュレーター (Rust + egui)");
            ui.separator();

            ui.collapsing("📥 物質の質量・環境パラメータの調整", |ui| {
                ui.label("【 溶媒 】");
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", self.water.name));
                    ui.add(egui::Slider::new(&mut self.water.mass, 0.0..=2000.0).suffix(" g"));
                });

                ui.label("【 溶質 】");
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", self.nacl.name));
                    ui.add(egui::Slider::new(&mut self.nacl.mass, 0.0..=300.0).suffix(" g"));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", self.sugar.name));
                    ui.add(egui::Slider::new(&mut self.sugar.mass, 0.0..=500.0).suffix(" g"));
                });

                ui.label("【 溶液環境 】");
                ui.horizontal(|ui| {
                    ui.label("溶液の仮想体積:");
                    ui.add(egui::Slider::new(&mut self.volume_liters, 0.1..=5.0).suffix(" L"));
                });
            });

            ui.add_space(10.0);

            let total_solute_mass = self.nacl.mass + self.sugar.mass;
            let total_mass = self.water.mass + total_solute_mass;
            
            let mass_percent = if total_mass > 0.0 {
                (total_solute_mass / total_mass) * 100.0
            } else {
                0.0
            };

            ui.heading("📊 リアルタイム解析結果");
            ui.separator();

            egui::Grid::new("result_grid")
                .num_columns(2)
                .spacing([40.0, 12.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("溶液総質量:");
                    ui.label(format!("{:.2} g", total_mass));
                    ui.end_row();

                    ui.label("質量パーセント濃度 (全体):");
                    ui.label(format!("{:.2} %", mass_percent));
                    ui.end_row();

                    ui.label(format!("{} のモル濃度:", self.nacl.name));
                    let nacl_molarity = self.nacl.moles() / self.volume_liters;
                    ui.label(format!("{:.3} mol/L  ({:.3} mol)", nacl_molarity, self.nacl.moles()));
                    ui.end_row();

                    ui.label(format!("{} のモル濃度:", self.sugar.name));
                    let sugar_molarity = self.sugar.moles() / self.volume_liters;
                    ui.label(format!("{:.3} mol/L  ({:.3} mol)", sugar_molarity, self.sugar.moles()));
                    ui.end_row();
                });

            if mass_percent > 35.0 {
                ui.add_space(15.0);
                ui.colored_label(
                    egui::Color32::LIGHT_RED,
                    "⚠️ 警告: 溶質濃度が非常に高くなっています。常温における溶解度の限界を超えて析出する可能性があります。",
                );
            }
        });
    }
}

// ==========================================
// 4. 日本語フォント初期化ロジック (型推論エラー修正版)
// ==========================================
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // OSのシステムフォントからゴシック体（日本語対応フォント）を検索
    let source = SystemSource::new();
    let mut font_data: Option<Vec<u8>> = None; // 型を明示

    let candidates = ["MS Gothic", "Hiragino Kaku Gothic ProN", "Noto Sans CJK JP", "Arial Unicode MS"];

    for family in candidates.iter() {
        if let Ok(handle) = source.select_best_match(
            &[font_kit::family_name::FamilyName::Title(family.to_string())],
            &Properties::new(),
        ) {
            if let Ok(font) = handle.load() {
                if let Some(data) = font.copy_font_data() {
                    // ここで Vec<u8> へ変換することを明示的に指定
                    font_data = Some((*data).clone());
                    break;
                }
            }
        }
    }

    // フォントデータが見つかった場合の処理
    if let Some(data) = font_data {
        fonts.font_data.insert(
            "japanese_font".to_owned(),
            egui::FontData::from_owned(data),
        );

        // プロポーショナル（標準テキスト）とモノスペース（等幅）の両方に割り当て
        fonts.families.get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "japanese_font".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "japanese_font".to_owned());
    }

    ctx.set_fonts(fonts);
}

// ==========================================
// 5. エントリーポイント
// ==========================================
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([550.0, 480.0])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "化学溶液シミュレーター",
        options,
        Box::new(|cc| {
            // アプリ起動時に日本語フォントを登録
            setup_custom_fonts(&cc.egui_ctx);
            
            // UIフォントサイズの一括調整
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles.insert(egui::TextStyle::Body, egui::FontId::new(15.0, egui::FontFamily::Proportional));
            style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(19.0, egui::FontFamily::Proportional));
            cc.egui_ctx.set_style(style);

            Box::new(ChemSimApp::default())
        }),
    )
}