//! 基础类型定义，作为 Qynapse 事件与策略的共同语言。
//!
//! 本模块定义了量化交易系统中的核心数据类型，确保类型安全、
//! 精度控制和语义清晰。所有类型都支持序列化和文档生成。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// 高精度价格类型，面向货币、点数等需要确定小数位的金融场景。
///
/// 使用 `rust_decimal::Decimal` 提供精确的十进制运算，避免浮点数精度问题。
/// 适用于股票价格、汇率、利率等金融数值的表示。
///
/// # 示例
/// ```
/// use qynapse_core::Px;
/// use rust_decimal::Decimal;
///
/// let price = Px(Decimal::from_str_exact("123.45").unwrap());
/// assert_eq!(price.0.to_string(), "123.45");
/// ```
///
/// # 注意
/// - 支持最多 28 位精度
/// - 序列化格式为字符串以避免精度丢失
/// - 实现了 Copy trait 以便高效传递
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Px(pub Decimal);

/// 交易数量类型，表示手数、张数、币数等交易单位。
///
/// 使用 `Decimal` 确保数量计算的精确性，支持大数量交易和分数交易。
/// 适用于股票手数、期货合约数、加密货币数量等场景。
///
/// # 示例
/// ```
/// use qynapse_core::Qty;
/// use rust_decimal::Decimal;
///
/// let quantity = Qty(Decimal::from(1000));
/// let fractional_qty = Qty(Decimal::from_str_exact("0.5").unwrap()); // 支持分数交易
/// ```
///
/// # 特性
/// - 支持整数和分数数量
/// - 适用于不同市场的最小交易单位要求
/// - 与价格类型配合进行金额计算
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Qty(pub Decimal);

/// 组合或策略中的权重类型，通常在 \[0, 1\] 区间内。
///
/// 用于表示资产在投资组合中的配置比例，或信号在策略中的权重。
/// 确保权重计算的数学正确性和边界检查。
///
/// # 示例
/// ```rust
/// use qynapse_core::Wgt;
/// use rust_decimal::Decimal;
///
/// let full_allocation = Wgt(Decimal::from(1));   // 100% 配置
/// let half_allocation = Wgt(Decimal::from_str_exact("0.5").unwrap()); // 50% 配置
/// let cash_weight = Wgt(Decimal::from_str_exact("0.2").unwrap());     // 20% 现金
/// ```
///
/// # 验证
/// 建议在使用时进行权重和检查：
/// ```rust
/// use qynapse_core::Wgt;
/// use rust_decimal::Decimal;
/// fn validate_portfolio_weights(weights: &[Wgt]) -> bool {
///     let total: Decimal = weights.iter().map(|w| w.0).sum();
///     total == Decimal::from(1)
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Wgt(pub Decimal);

/// 标的唯一标识符，用于识别金融工具。
///
/// 表示股票代码、期货合约、期权代码、加密货币对等金融工具的全局唯一标识。
/// 采用字符串包装类型确保类型安全和语义清晰。
///
/// # 格式约定
/// - 股票: `"600519.SH"`, `"AAPL.US"`
/// - 期货: `"IF2401.CFFEX"`, `"CLF24.NYMEX"`
/// - 加密货币: `"BTC-USDT"`, `"ETH-USD"`
/// - 外汇: `"EUR/USD"`, `"USD/JPY"`
///
/// # 示例
/// ```
/// use qynapse_core::InstrumentId;
///
/// let stock = InstrumentId("600519.SH".to_string());  // 贵州茅台
/// let future = InstrumentId("IF2401.CFFEX".to_string()); // 沪深300期货
/// let crypto = InstrumentId("BTC-USDT".to_string());     // 比特币
/// ```
///
/// # 注意
/// - 实现了 Hash、Eq、PartialEq，可用于 HashMap 键值
/// - 建议使用标准化代码格式
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct InstrumentId(pub String);

/// 交易场所标识符，区分同一标的在不同市场的报价和交易。
///
/// 在现代交易生态中，同一金融工具可能在多个不同类型的交易场所进行交易，
/// 每个场所具有不同的流动性特征、交易规则和费用结构。
///
/// # 场所类型的重要性
/// - **流动性分析**: 不同场所的流动性特征差异巨大
/// - **智能订单路由**: 根据场所类型选择最优执行路径
/// - **监管合规**: MiFID II 等法规要求区分场所类型
/// - **费用计算**: 不同场所的费用结构不同
///
/// # 示例
/// ```
/// use qynapse_core::{Venue, VenueType};
///
/// // 传统交易所
/// let sse = Venue {
///     id: "XSHG".to_string(),
///     name: "上海证券交易所".to_string(),
///     venue_type: VenueType::Exchange,
///     country: "CN".to_string(),
///     mic: Some("XSHG".to_string()),
///     timezone: "Asia/Shanghai".to_string(),
/// };
///
/// // 暗池交易
/// let ms_dark = Venue {
///     id: "MSDARK".to_string(),
///     name: "Morgan Stanley Dark Pool".to_string(),
///     venue_type: VenueType::DarkPool,
///     country: "US".to_string(),
///     mic: None,
///     timezone: "America/New_York".to_string(),
/// };
///
/// // 加密货币交易所
/// let binance = Venue {
///     id: "BINANCE".to_string(),
///     name: "Binance Global".to_string(),
///     venue_type: VenueType::Exchange,
///     country: "Global".to_string(),
///     mic: None,
///     timezone: "UTC".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    /// 场所唯一标识符，通常使用标准化的代码系统
    ///
    /// # 常用标识符
    /// - **MIC (Market Identifier Code)**: `"XNAS"`, `"XNYS"`, `"XSHG"`
    /// - **自定义代码**: `"BINANCE"`, `"MSDARK"`, `"ICAP"`
    /// - **内部代码**: `"INTERNAL_CROSS"`, `"BROKER_INTERNAL"`
    pub id: String,

    /// 场所全称，便于显示和日志记录
    ///
    /// # 示例
    /// - `"Nasdaq Stock Market"`
    /// - `"Shanghai Stock Exchange"`
    /// - `"Morgan Stanley Dark Pool"`
    pub name: String,

    /// 场所类型，决定交易特性和监管要求
    ///
    /// 这是最重要的字段之一，影响订单路由、费用计算和合规处理。
    pub venue_type: VenueType,

    /// 所在国家或地区代码，遵循 ISO 3166-1 alpha-2 标准
    ///
    /// # 示例
    /// - `"US"` - 美国
    /// - `"CN"` - 中国
    /// - `"HK"` - 香港
    /// - `"JP"` - 日本
    /// - `"Global"` - 全球性场所（如加密货币交易所）
    pub country: String,

    /// ISO 10383 市场识别码 (MIC)，适用于受监管场所
    ///
    /// 对于受监管的交易所和 MTF，应提供标准的 MIC 代码。
    /// 对于暗池、经纪商内部撮合等非标准场所，此字段为 None。
    ///
    /// # 参考
    /// - [ISO 10383 标准](https://www.iso20022.org/10383/iso-10383-market-identifier-codes)
    pub mic: Option<String>,

    /// 场所所在时区，用于时间标准化
    ///
    /// 使用 IANA 时区数据库格式，如 `"America/New_York"`, `"Asia/Shanghai"`。
    /// 对于全球性场所使用 `"UTC"`。
    pub timezone: String,
}

/// 交易场所类型分类，基于监管性质和交易特性。
///
/// 此分类遵循 MiFID II 等现代金融监管框架，对于最佳执行、
/// 交易报告和风险管理至关重要。
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VenueType {
    /// 受监管的交易所
    ///
    /// 具有中央对手方清算、公开订单簿和严格监管的传统交易所。
    ///
    /// # 示例
    /// - 上海证券交易所 (XSHG)
    /// - 纳斯达克 (XNAS)
    /// - 芝加哥商品交易所 (XCME)
    /// - 伦敦证券交易所 (XLON)
    Exchange,

    /// 多边交易设施 (MTF) - 欧洲监管框架
    ///
    /// 非交易所但受监管的多边交易系统，具有透明交易规则。
    ///
    /// # 示例
    /// - Chi-X Europe
    /// - BATS Europe
    /// - Turquoise
    MultilateralTradingFacility,

    /// 有组织交易设施 (OTF) - MiFID II 新增
    ///
    /// 适用于非权益类工具的多边交易系统，具有更多灵活性。
    ///
    /// # 示例
    /// - 银行间的债券交易平台
    /// - 衍生品交易设施
    OrganisedTradingFacility,

    /// 另类交易系统 (ATS) - 美国监管框架
    ///
    /// 美国的非交易所交易系统，功能类似于欧洲的 MTF。
    ///
    /// # 示例
    /// - Liquidnet
    /// - POSIT
    /// - Crossfinder
    AlternativeTradingSystem,

    /// 电子通讯网络 (ECN)
    ///
    /// 自动匹配买卖订单的电子系统，通常显示深度行情。
    ///
    /// # 示例
    /// - Instinet
    /// - Bloomberg Tradebook
    /// - NYFIX Millennium
    ElectronicCommunicationNetwork,

    /// 暗池
    ///
    /// 不公开显示订单信息的私人交易场所，提供大额交易匿名性。
    ///
    /// # 特性
    /// - 不预先显示流动性
    /// - 通常提供价格改善
    /// - 适合大额订单执行
    ///
    /// # 示例
    /// - Sigma X (Goldman Sachs)
    /// - MS Pool (Morgan Stanley)
    /// - Crossstream (Credit Suisse)
    DarkPool,

    /// 经纪商内部撮合
    ///
    /// 经纪商将客户订单与自有库存或其他客户订单匹配。
    ///
    /// # 监管要求
    /// - 通常需要客户同意
    /// - 必须提供价格改善
    /// - 需要详细报告
    BrokerInternalization,

    /// 系统性内部化 (SI) - 欧洲监管框架
    ///
    /// 投资公司使用自有资本与客户订单持续交易。
    ///
    /// # 要求
    /// - 必须对特定工具提供连续报价
    /// - 受流动性提供规则约束
    SystematicInternaliser,

    /// 场外交易 (OTC)
    ///
    /// 双边协商交易，无中央订单簿。
    ///
    /// # 适用产品
    /// - 外汇
    /// - 结构性产品
    /// - 非标准化衍生品
    OverTheCounter,

    /// 自营交易平台
    ///
    /// 主要用于公司自有账户交易，可能也向客户提供流动性。
    ProprietaryTradingPlatform,
}

/// 货币代码类型，遵循 ISO 4217 标准的三字母代码。
///
/// 用于表示交易货币、结算货币、计价货币等。
/// 确保货币相关计算的类型安全和汇率转换的正确性。
///
/// # 常用货币代码
/// - `"CNY"` - 人民币
/// - `"USD"` - 美元
/// - `"EUR"` - 欧元
/// - `"JPY"` - 日元
/// - `"HKD"` - 港币
///
/// # 示例
/// ```
/// use qynapse_core::Currency;
///
/// let us_dollar = Currency("USD".to_string());
/// let chinese_yuan = Currency("CNY".to_string());
/// let euro = Currency("EUR".to_string());
/// ```
///
/// # 标准遵循
/// - ISO 4217 三字母代码
/// - 加密货币可使用非标准代码，如 `"BTC"`, `"ETH"`
/// - 建议统一使用大写字母
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Currency(pub String);

/// 多时间戳结构，记录事件在交易流水线中各节点的观测时间。
///
/// 在分布式交易系统中，同一事件在不同处理节点的时间戳对于
/// 延迟分析、排序逻辑和监管合规至关重要。
///
/// # 时间轴说明
/// ```text
/// 交易所时间 (ts_exchange) → 网关时间 (ts_gateway) → 本地时间 (ts_local)
///      ↓              ↓                   ↓
///   撮合时间      网络接收时间        策略处理时间
/// ```
///
/// # 示例
/// ```
/// use qynapse_core::TimeStamps;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let timestamps = TimeStamps {
///     ts_exchange: SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(1672531200000),
///     ts_gateway: Some(SystemTime::now()),
///     ts_local: Some(SystemTime::now()),
/// };
/// ```
///
/// # 时间延迟分析
/// - **网络延迟**: `ts_gateway - ts_exchange`
/// - **处理延迟**: `ts_local - ts_gateway`
/// - **总延迟**: `ts_local - ts_exchange`
///
/// # 注意
/// - `ts_exchange` 必须提供，作为事件的主时间参考
/// - `ts_gateway` 和 `ts_local` 为可选，但建议在可能的情况下记录
/// - 所有时间戳使用 SystemTime，支持跨平台和序列化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStamps {
    /// 交易所撮合时间或数据源产生时间，作为事件的主时间轴基准。
    ///
    /// 这是最具权威性的时间戳，通常来自交易所的官方时钟。
    /// 用于排序、回放和合规报告。
    pub ts_exchange: SystemTime,

    /// 网关或数据接口接收到事件的时间。
    ///
    /// 反映网络传输延迟，用于监控数据链路质量。
    /// 对于本地模拟数据，此字段可为 None。
    pub ts_gateway: Option<SystemTime>,

    /// 本地策略引擎处理事件的时间。
    ///
    /// 用于评估策略逻辑的处理延迟和性能分析。
    /// 在回测模式中，此字段通常与交易所时间相同。
    pub ts_local: Option<SystemTime>,
}

// =============================================================================
// Px 类型的实现
// =============================================================================

impl Px {
    /// 创建零价格
    pub fn zero() -> Self {
        Px(Decimal::ZERO)
    }

    /// 检查价格是否为正数
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }

    /// 检查价格是否为负数
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    /// 检查价格是否为零
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// 计算价格变化百分比
    ///
    /// # 参数
    /// - `previous`: 前一个价格
    ///
    /// # 返回值
    /// - `Some(percent)`: 价格变化百分比
    /// - `None`: 如果前一个价格为零（除零错误）
    pub fn percent_change(&self, previous: &Px) -> Option<Decimal> {
        if previous.0.is_zero() {
            None
        } else {
            Some((self.0 - previous.0) / previous.0 * Decimal::from(100))
        }
    }

    /// 计算价格绝对变化
    pub fn absolute_change(&self, previous: &Px) -> Decimal {
        self.0 - previous.0
    }

    /// 获取价格的绝对值
    pub fn abs(&self) -> Self {
        Px(self.0.abs())
    }
}
impl Default for Px {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Qty 类型的实现
// =============================================================================

impl Qty {
    /// 创建零数量
    pub fn zero() -> Self {
        Qty(Decimal::ZERO)
    }

    /// 检查数量是否为正（买入方向）
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }

    /// 检查数量是否为负（卖出方向）
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    /// 检查数量是否为零
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// 计算绝对数量
    pub fn abs(&self) -> Self {
        Qty(self.0.abs())
    }

    /// 计算总价值（价格 × 数量）
    pub fn value(&self, price: &Px) -> Decimal {
        self.0 * price.0
    }
}

impl Default for Qty {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Wgt 类型的实现
// =============================================================================

impl Wgt {
    /// 创建零权重
    pub fn zero() -> Self {
        Wgt(Decimal::ZERO)
    }

    /// 创建全权重 (1.0)
    pub fn one() -> Self {
        Wgt(Decimal::ONE)
    }

    /// 检查权重是否在有效范围内 [0, 1]
    pub fn is_valid(&self) -> bool {
        self.0 >= Decimal::ZERO && self.0 <= Decimal::ONE
    }

    /// 检查权重是否超出有效范围
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// 转换为百分比表示
    pub fn as_percent(&self) -> Decimal {
        self.0 * Decimal::from(100)
    }

    /// 从百分比创建权重
    pub fn from_percent(percent: Decimal) -> Self {
        Wgt(percent / Decimal::from(100))
    }

    /// 检查权重是否表示全仓（100%）
    pub fn is_full_allocation(&self) -> bool {
        self.0 == Decimal::ONE
    }

    /// 检查权重是否表示空仓（0%）
    pub fn is_zero_allocation(&self) -> bool {
        self.0.is_zero()
    }
}

impl Default for Wgt {
    fn default() -> Self {
        Self::zero()
    }
}

// =============================================================================
// Venue 类型的实现
// =============================================================================

impl Venue {
    /// 创建新的交易场所实例
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        venue_type: VenueType,
        country: impl Into<String>,
        timezone: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            venue_type,
            country: country.into(),
            mic: None,
            timezone: timezone.into(),
        }
    }

    /// 设置 MIC 代码
    pub fn with_mic(mut self, mic: impl Into<String>) -> Self {
        self.mic = Some(mic.into());
        self
    }

    /// 检查是否为受监管场所
    ///
    /// 受监管场所包括交易所、MTF、OTF 等需要正式监管报告的场所。
    pub fn is_regulated(&self) -> bool {
        matches!(
            self.venue_type,
            VenueType::Exchange
                | VenueType::MultilateralTradingFacility
                | VenueType::OrganisedTradingFacility
                | VenueType::SystematicInternaliser
        )
    }

    /// 检查是否显示订单簿（非暗池）
    ///
    /// 透明场所会公开显示订单簿信息，适合需要市场深度的策略。
    pub fn is_transparent(&self) -> bool {
        !matches!(
            self.venue_type,
            VenueType::DarkPool | VenueType::BrokerInternalization
        )
    }

    /// 检查是否提供价格改善义务
    ///
    /// 某些场所类型（如内部撮合）有义务提供价格改善。
    pub fn has_price_improvement_obligation(&self) -> bool {
        matches!(
            self.venue_type,
            VenueType::BrokerInternalization | VenueType::SystematicInternaliser
        )
    }

    /// 获取场所的标准显示名称
    ///
    /// 优先使用 MIC 代码，如果没有则使用 ID。
    pub fn display_name(&self) -> &str {
        self.mic.as_deref().unwrap_or(&self.id)
    }

    /// 检查是否为暗池类型
    pub fn is_dark_pool(&self) -> bool {
        matches!(self.venue_type, VenueType::DarkPool)
    }

    /// 检查是否为内部撮合类型
    pub fn is_internalization(&self) -> bool {
        matches!(
            self.venue_type,
            VenueType::BrokerInternalization | VenueType::SystematicInternaliser
        )
    }
}

// 为常见场所提供便捷构造函数
impl Venue {
    /// 创建主要股票交易所
    pub fn major_exchange(id: &str, name: &str, country: &str, timezone: &str) -> Self {
        Self::new(id, name, VenueType::Exchange, country, timezone).with_mic(id) // 主要交易所通常 ID 就是 MIC
    }

    /// 创建暗池场所
    pub fn dark_pool(id: &str, name: &str, country: &str) -> Self {
        Self::new(id, name, VenueType::DarkPool, country, "UTC")
    }

    /// 创建加密货币交易所
    pub fn crypto_exchange(id: &str, name: &str) -> Self {
        Self::new(id, name, VenueType::Exchange, "Global", "UTC")
    }

    /// 创建经纪商内部撮合场所
    pub fn broker_internalization(id: &str, name: &str, country: &str) -> Self {
        Self::new(id, name, VenueType::BrokerInternalization, country, "UTC")
    }
}

// =============================================================================
// TimeStamps 类型的实现
// =============================================================================

impl TimeStamps {
    /// 创建仅包含交易所时间戳的简化实例
    pub fn exchange_only(ts_exchange: SystemTime) -> Self {
        Self {
            ts_exchange,
            ts_gateway: None,
            ts_local: None,
        }
    }

    /// 创建包含所有时间戳的完整实例
    pub fn full(ts_exchange: SystemTime, ts_gateway: SystemTime, ts_local: SystemTime) -> Self {
        Self {
            ts_exchange,
            ts_gateway: Some(ts_gateway),
            ts_local: Some(ts_local),
        }
    }

    /// 计算网关处理延迟（微秒）
    ///
    /// # 返回值
    /// - `Some(latency)`: 网关延迟（微秒）
    /// - `None`: 如果没有网关时间戳
    pub fn gateway_latency_us(&self) -> Option<u128> {
        self.ts_gateway?
            .duration_since(self.ts_exchange)
            .ok()
            .map(|d| d.as_micros())
    }

    /// 计算本地处理延迟（微秒）
    ///
    /// # 返回值
    /// - `Some(latency)`: 本地处理延迟（微秒）
    /// - `None`: 如果没有本地时间戳
    pub fn local_latency_us(&self) -> Option<u128> {
        self.ts_local?
            .duration_since(self.ts_exchange)
            .ok()
            .map(|d| d.as_micros())
    }

    /// 计算总延迟（从交易所到本地处理，微秒）
    ///
    /// # 返回值
    /// - `Some(latency)`: 总延迟（微秒）
    /// - `None`: 如果没有本地时间戳
    pub fn total_latency_us(&self) -> Option<u128> {
        self.ts_local?
            .duration_since(self.ts_exchange)
            .ok()
            .map(|d| d.as_micros())
    }

    /// 获取最可靠的时间戳（优先使用交易所时间）
    pub fn best_timestamp(&self) -> SystemTime {
        self.ts_exchange
    }

    /// 检查时间戳是否按时间顺序排列（交易所 → 网关 → 本地）
    pub fn is_chronological(&self) -> bool {
        let mut valid = true;

        if let Some(ts_gateway) = self.ts_gateway {
            valid &= ts_gateway >= self.ts_exchange;
        }

        if let Some(ts_local) = self.ts_local {
            if let Some(ts_gateway) = self.ts_gateway {
                valid &= ts_local >= ts_gateway;
            } else {
                valid &= ts_local >= self.ts_exchange;
            }
        }

        valid
    }
}

impl Default for TimeStamps {
    fn default() -> Self {
        Self {
            ts_exchange: SystemTime::now(),
            ts_gateway: None,
            ts_local: None,
        }
    }
}

// =============================================================================
// From trait 实现
// =============================================================================

impl From<&str> for InstrumentId {
    fn from(s: &str) -> Self {
        InstrumentId(s.to_string())
    }
}

impl From<String> for InstrumentId {
    fn from(s: String) -> Self {
        InstrumentId(s)
    }
}

impl From<&str> for Venue {
    fn from(id: &str) -> Self {
        // 根据常见 ID 推断类型和名称
        let (name, venue_type, country, timezone) = match id {
            "XSHG" => ("上海证券交易所", VenueType::Exchange, "CN", "Asia/Shanghai"),
            "XSHE" => ("深圳证券交易所", VenueType::Exchange, "CN", "Asia/Shanghai"),
            "XNAS" => (
                "纳斯达克交易所",
                VenueType::Exchange,
                "US",
                "America/New_York",
            ),
            "XNYS" => (
                "纽约证券交易所",
                VenueType::Exchange,
                "US",
                "America/New_York",
            ),
            "BINANCE" => ("Binance", VenueType::Exchange, "Global", "UTC"),
            "MSDARK" => (
                "Morgan Stanley Dark Pool",
                VenueType::DarkPool,
                "US",
                "America/New_York",
            ),
            _ => (id, VenueType::Exchange, "US", "UTC"), // 默认为美国交易所
        };

        Venue::new(id, name, venue_type, country, timezone)
    }
}

impl From<String> for Venue {
    fn from(s: String) -> Self {
        Venue::from(s.as_str())
    }
}

impl From<&str> for Currency {
    fn from(s: &str) -> Self {
        Currency(s.to_uppercase())
    }
}

impl From<String> for Currency {
    fn from(s: String) -> Self {
        Currency(s.to_uppercase())
    }
}

// =============================================================================
// 标准 trait 实现
// =============================================================================

impl std::fmt::Display for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Venue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for VenueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            VenueType::Exchange => "Exchange",
            VenueType::MultilateralTradingFacility => "MTF",
            VenueType::OrganisedTradingFacility => "OTF",
            VenueType::AlternativeTradingSystem => "ATS",
            VenueType::ElectronicCommunicationNetwork => "ECN",
            VenueType::DarkPool => "DarkPool",
            VenueType::BrokerInternalization => "BrokerInternal",
            VenueType::SystematicInternaliser => "SystematicInternaliser",
            VenueType::OverTheCounter => "OTC",
            VenueType::ProprietaryTradingPlatform => "Proprietary",
        };
        write!(f, "{}", name)
    }
}

// =============================================================================
// 测试模块
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn px_serializes_as_string() {
        let px = Px(Decimal::from_str_exact("123.456789").unwrap());
        let json = serde_json::to_string(&px).unwrap();
        assert_eq!(json, "\"123.456789\"");
    }

    #[test]
    fn px_deserializes_from_string() {
        let raw = "\"987.654321\"";
        let px: Px = serde_json::from_str(raw).unwrap();
        assert_eq!(px.0, Decimal::from_str_exact("987.654321").unwrap());
    }

    #[test]
    fn test_px_operations() {
        let px1 = Px(dec!(100.0));
        let px2 = Px(dec!(110.0));
        let percent = px1.percent_change(&px2).unwrap();

        assert_eq!(percent.round_dp(12), dec!(-9.090909090909));
        assert!(px1.is_positive());
        assert!(!px1.is_negative());
        assert!(!px1.is_zero());
    }

    #[test]
    fn test_qty_direction() {
        let buy_qty = Qty(dec!(100));
        let sell_qty = Qty(dec!(-100));
        let zero_qty = Qty(dec!(0));

        assert!(buy_qty.is_positive());
        assert!(sell_qty.is_negative());
        assert!(zero_qty.is_zero());
        assert!(!zero_qty.is_positive());
        assert_eq!(sell_qty.abs().0, dec!(100));
    }

    #[test]
    fn test_weight_validation() {
        let valid_weight = Wgt(dec!(0.5));
        let invalid_weight = Wgt(dec!(1.5));
        let zero_weight = Wgt::zero();
        let full_weight = Wgt::one();

        assert!(valid_weight.is_valid());
        assert!(!invalid_weight.is_valid());
        assert_eq!(valid_weight.as_percent(), dec!(50));
        assert!(zero_weight.is_zero_allocation());
        assert!(full_weight.is_full_allocation());
    }

    #[test]
    fn test_venue_creation() {
        let sse = Venue::major_exchange("XSHG", "上海证券交易所", "CN", "Asia/Shanghai");
        assert_eq!(sse.id, "XSHG");
        assert_eq!(sse.venue_type, VenueType::Exchange);
        assert!(sse.is_regulated());
        assert!(sse.is_transparent());
        assert_eq!(sse.display_name(), "XSHG");
    }

    #[test]
    fn test_dark_pool_characteristics() {
        let dark_pool = Venue::dark_pool("MSDARK", "Morgan Stanley Dark Pool", "US");
        assert!(!dark_pool.is_transparent());
        assert!(!dark_pool.is_regulated());
        assert!(dark_pool.is_dark_pool());
    }

    #[test]
    fn test_venue_from_str() {
        let venue: Venue = "XSHG".into();
        assert_eq!(venue.id, "XSHG");
        assert_eq!(venue.venue_type, VenueType::Exchange);
        assert_eq!(venue.country, "CN");
    }

    #[test]
    fn test_timestamps_latency() {
        let exchange_time = UNIX_EPOCH + Duration::from_millis(1000);
        let gateway_time = UNIX_EPOCH + Duration::from_millis(1005);
        let local_time = UNIX_EPOCH + Duration::from_millis(1010);

        let ts = TimeStamps {
            ts_exchange: exchange_time,
            ts_gateway: Some(gateway_time),
            ts_local: Some(local_time),
        };

        assert_eq!(ts.gateway_latency_us(), Some(5000)); // 5ms
        assert_eq!(ts.local_latency_us(), Some(10000)); // 10ms
        assert_eq!(ts.total_latency_us(), Some(10000)); // 10ms
        assert!(ts.is_chronological());
    }

    #[test]
    fn test_currency_uppercase() {
        let currency: Currency = "usd".into();
        assert_eq!(currency.0, "USD");
    }

    #[test]
    fn test_instrument_id_display() {
        let inst = InstrumentId("AAPL.US".to_string());
        assert_eq!(format!("{}", inst), "AAPL.US");
    }

    #[test]
    fn test_venue_type_display() {
        assert_eq!(format!("{}", VenueType::Exchange), "Exchange");
        assert_eq!(format!("{}", VenueType::DarkPool), "DarkPool");
    }
}
