// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "invoice_payment_method_options_us_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccount {
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<InvoicePaymentMethodOptionsUsBankAccountVerificationMethod>,
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => "automatic",
            InvoicePaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            InvoicePaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}
