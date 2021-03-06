//  Copyright (C) 2018  The Duniter Project Developers.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use constants::*;
use duniter_documents::blockchain::v10::documents::block::{BlockV10Parameters, CurrencyName};
use *;

#[derive(Debug, Copy, Clone)]
/// Curerncy parameters
pub struct CurrencyParameters {
    /// Protocol version
    pub protocol_version: usize,
    /// UD target growth rate (see Relative Theorie of Money)
    pub c: f64,
    /// Duration between the creation of two UD (in seconds)
    pub dt: u64,
    /// Amount of the initial UD
    pub ud0: usize,
    /// Minimum duration between the writing of 2 certifications from the same issuer (in seconds)
    pub sig_period: u64,
    /// Minimum duration between two renewals of the same certification
    pub sig_renew_period: u64,
    /// Maximum number of active certifications at the same time (for the same issuer)
    pub sig_stock: usize,
    /// Maximum retention period of a pending certification
    pub sig_window: u64,
    /// Time to expiry of written certification
    pub sig_validity: u64,
    /// Minimum number of certifications required to become a member
    pub sig_qty: usize,
    /// Maximum retention period of a pending identity
    pub idty_window: u64,
    /// Maximum retention period of a pending membership
    pub ms_window: u64,
    /// Maximum retention period of a pending transaction
    pub tx_window: u64,
    /// Percentage of referring members who must be within step_max steps of each member
    pub x_percent: f64,
    /// Time to expiry of written membership
    pub ms_validity: u64,
    /// Minimum duration between the writing of 2 memberships from the same issuer (in seconds)
    pub ms_period: u64,
    /// For a member to respect the distance rule,
    /// there must exist for more than x_percent % of the referring members
    /// a path of less than step_max steps from the referring member to the evaluated member.
    pub step_max: usize,
    /// Number of blocks used for calculating median time.
    pub median_time_blocks: usize,
    /// The average time for writing 1 block (wished time)
    pub avg_gen_time: u64,
    /// The number of blocks required to evaluate again PoWMin value
    pub dt_diff_eval: usize,
    /// The percent of previous issuers to reach for personalized difficulty
    pub percent_rot: f64,
    /// Time of first UD.
    pub ud_time0: u64,
    /// Time of first reevaluation of the UD.
    pub ud_reeval_time0: u64,
    /// Time period between two re-evaluation of the UD.
    pub dt_reeval: u64,
}

impl From<(CurrencyName, BlockV10Parameters)> for CurrencyParameters {
    fn from(source: (CurrencyName, BlockV10Parameters)) -> CurrencyParameters {
        let (currency_name, block_params) = source;
        let sig_renew_period = match currency_name.0.as_str() {
            "default_currency" => *DEFAULT_SIG_RENEW_PERIOD,
            "g1" => 5_259_600,
            "g1-test" => 5_259_600 / 5,
            _ => *DEFAULT_SIG_RENEW_PERIOD,
        };
        let ms_period = match currency_name.0.as_str() {
            "default_currency" => *DEFAULT_MS_PERIOD,
            "g1" => 5_259_600,
            "g1-test" => 5_259_600 / 5,
            _ => *DEFAULT_MS_PERIOD,
        };
        let tx_window = match currency_name.0.as_str() {
            "default_currency" => *DEFAULT_TX_WINDOW,
            "g1" => 604_800,
            "g1-test" => 604_800,
            _ => *DEFAULT_TX_WINDOW,
        };
        CurrencyParameters {
            protocol_version: 10,
            c: block_params.c,
            dt: block_params.dt,
            ud0: block_params.ud0,
            sig_period: block_params.sig_period,
            sig_renew_period,
            sig_stock: block_params.sig_stock,
            sig_window: block_params.sig_window,
            sig_validity: block_params.sig_validity,
            sig_qty: block_params.sig_qty,
            idty_window: block_params.idty_window,
            ms_window: block_params.ms_window,
            tx_window,
            x_percent: block_params.x_percent,
            ms_validity: block_params.ms_validity,
            ms_period,
            step_max: block_params.step_max,
            median_time_blocks: block_params.median_time_blocks,
            avg_gen_time: block_params.avg_gen_time,
            dt_diff_eval: block_params.dt_diff_eval,
            percent_rot: block_params.percent_rot,
            ud_time0: block_params.ud_time0,
            ud_reeval_time0: block_params.ud_reeval_time0,
            dt_reeval: block_params.dt_reeval,
        }
    }
}

impl Default for CurrencyParameters {
    fn default() -> CurrencyParameters {
        CurrencyParameters::from((
            CurrencyName(String::from("default_currency")),
            BlockV10Parameters::default(),
        ))
    }
}

impl CurrencyParameters {
    /// Get max value of connectivity (=1/x_percent)
    pub fn max_connectivity(&self) -> f64 {
        1.0 / self.x_percent
    }
}

/// Get currency parameters
pub fn get_currency_params(
    blockchain_db: &BinDB<LocalBlockchainV10Datas>,
) -> Result<Option<CurrencyParameters>, DALError> {
    Ok(blockchain_db.read(|db| {
        if let Some(genesis_block) = db.get(&BlockId(0)) {
            if genesis_block.block.parameters.is_some() {
                Some(CurrencyParameters::from((
                    genesis_block.block.currency.clone(),
                    genesis_block.block.parameters.expect("safe unwrap"),
                )))
            } else {
                panic!("The genesis block are None parameters !");
            }
        } else {
            None
        }
    })?)
}
