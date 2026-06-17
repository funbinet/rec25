/// Kinds of input a mode can require from the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Domain,
    Ip,
    Url,
    File,
    Target,
    Ports,
    FreeText,
}

impl InputKind {
    /// Human-readable prompt label shown to the user.
    pub fn label(self) -> &'static str {
        match self {
            Self::Domain   => "Domain",
            Self::Ip       => "IP / CIDR",
            Self::Url      => "URL",
            Self::File     => "File path",
            Self::Target   => "Target",
            Self::Ports    => "Ports (e.g. 80,443)",
            Self::FreeText => "Value",
        }
    }

    /// Placeholder key used inside cmd_template strings.
    pub fn placeholder(self) -> &'static str {
        match self {
            Self::Domain   => "domain",
            Self::Ip       => "ip",
            Self::Url      => "url",
            Self::File     => "file",
            Self::Target   => "target",
            Self::Ports    => "ports",
            Self::FreeText => "value",
        }
    }
}

/// Expected output format — drives parser selection and file extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Newline-separated list of values (subdomains, IPs, URLs …).
    Lines,
    /// JSON document.
    Json,
    /// XML document.
    Xml,
    /// Nmap-style output (open ports, service info).
    Nmap,
    /// Comma-separated values.
    Csv,
    /// Unstructured / binary — display raw.
    Raw,
}

/// One operational mode for a security tool.
pub struct Mode {
    /// Short, descriptive name shown in the menu (2–4 words).
    pub name: &'static str,
    /// Shell command template.  Placeholders: {domain}, {ip}, {url},
    /// {file}, {target}, {ports}, {value}, {output_file}, {timestamp}.
    pub cmd_template: &'static str,
    /// Ordered list of inputs the user must supply before the command runs.
    pub inputs: &'static [InputKind],
    /// How the tool's output should be parsed and displayed.
    pub output_format: OutputFormat,
    /// File extension for the saved output file.
    pub file_ext: &'static str,
}

/// A security tool with one or more operational modes.
pub struct Tool {
    /// Display name shown in the menu.
    pub name: &'static str,
    /// Executable name looked up in PATH.
    pub binary: &'static str,
    /// At least four distinct modes.
    pub modes: &'static [Mode],
}

/// A top-level category grouping related tools.
pub struct Category {
    /// Display name shown in the category menu.
    pub name: &'static str,
    /// All tools belonging to this category.
    pub tools: &'static [Tool],
}
