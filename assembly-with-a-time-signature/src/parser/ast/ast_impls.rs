#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
// |==========================================================|
// |      WARNING: THIS FILE IS AUTOMATICALLY GENERATED.      |
// |      CHANGES TO IT WILL BE DELETED WHEN REGENERATED.     |
// | IN GENERAL, THIS FILE SHOULD NOT BE MODIFIED IN ANY WAY. |
// |==========================================================|
use super::prelude::*;
impl<M: AstInfo> AstNode<M> for LabelOrInstr<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::Instruction(meta, ..) => meta,
            Self::Label(meta, ..) => meta,
            Self::Combined(meta, ..) => meta,
            Self::Section(meta, ..) => meta,
            Self::Newline(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::Instruction(..) => "instruction",
            Self::Label(..) => "label",
            Self::Combined(..) => "combined",
            Self::Section(..) => "section",
            Self::Newline(..) => "newline",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "label-or-instr"
    }
}
impl<M: AstInfo> AstNode<M> for Label<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::Global(meta, ..) => meta,
            Self::Local(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::Global(..) => "global",
            Self::Local(..) => "local",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "label"
    }
}
impl<M: AstInfo> AstNode<M> for Instruction<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::Call(meta, ..) => meta,
            Self::CallAsync(meta, ..) => meta,
            Self::Ret(meta, ..) => meta,
            Self::RetAsync(meta, ..) => meta,
            Self::Jmp(meta, ..) => meta,
            Self::Bpm(meta, ..) => meta,
            Self::Time(meta, ..) => meta,
            Self::Beat(meta, ..) => meta,
            Self::Wait(meta, ..) => meta,
            Self::PlayOne(meta, ..) => meta,
            Self::Play(meta, ..) => meta,
            Self::Flat(meta, ..) => meta,
            Self::Sharp(meta, ..) => meta,
            Self::Push(meta, ..) => meta,
            Self::Pop(meta, ..) => meta,
            Self::Add(meta, ..) => meta,
            Self::Sub(meta, ..) => meta,
            Self::Inc(meta, ..) => meta,
            Self::Dec(meta, ..) => meta,
            Self::Jg(meta, ..) => meta,
            Self::Jl(meta, ..) => meta,
            Self::Jge(meta, ..) => meta,
            Self::Jle(meta, ..) => meta,
            Self::Jeq(meta, ..) => meta,
            Self::Je(meta, ..) => meta,
            Self::Jne(meta, ..) => meta,
            Self::Mov(meta, ..) => meta,
            Self::St(meta, ..) => meta,
            Self::Ld(meta, ..) => meta,
            Self::Define(meta, ..) => meta,
            Self::Lir(meta, ..) => meta,
            Self::Stop(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::Call(..) => "call",
            Self::CallAsync(..) => "call_async",
            Self::Ret(..) => "ret",
            Self::RetAsync(..) => "ret_async",
            Self::Jmp(..) => "jmp",
            Self::Bpm(..) => "bpm",
            Self::Time(..) => "time",
            Self::Beat(..) => "beat",
            Self::Wait(..) => "wait",
            Self::PlayOne(..) => "play_one",
            Self::Play(..) => "play",
            Self::Flat(..) => "flat",
            Self::Sharp(..) => "sharp",
            Self::Push(..) => "push",
            Self::Pop(..) => "pop",
            Self::Add(..) => "add",
            Self::Sub(..) => "sub",
            Self::Inc(..) => "inc",
            Self::Dec(..) => "dec",
            Self::Jg(..) => "jg",
            Self::Jl(..) => "jl",
            Self::Jge(..) => "jge",
            Self::Jle(..) => "jle",
            Self::Jeq(..) => "jeq",
            Self::Je(..) => "je",
            Self::Jne(..) => "jne",
            Self::Mov(..) => "mov",
            Self::St(..) => "st",
            Self::Ld(..) => "ld",
            Self::Define(..) => "define",
            Self::Lir(..) => "lir",
            Self::Stop(..) => "stop",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "instruction"
    }
}
impl<M: AstInfo> AstNode<M> for Section<M> {
    fn ast_info(&self) -> &M {
        let Self(meta, ..) = self;
        meta
    }
    fn constructor(&self) -> &'static str {
        "section"
    }
    fn sort(&self) -> &'static str {
        "section"
    }
}
impl<M: AstInfo> AstNode<M> for Int<M> {
    fn ast_info(&self) -> &M {
        let Self(meta, ..) = self;
        meta
    }
    fn constructor(&self) -> &'static str {
        "int"
    }
    fn sort(&self) -> &'static str {
        "int"
    }
}
impl<M: AstInfo> AstNode<M> for Register<M> {
    fn ast_info(&self) -> &M {
        let Self(meta, ..) = self;
        meta
    }
    fn constructor(&self) -> &'static str {
        "register"
    }
    fn sort(&self) -> &'static str {
        "register"
    }
}
impl<M: AstInfo> AstNode<M> for Note<M> {
    fn ast_info(&self) -> &M {
        let Self(meta, ..) = self;
        meta
    }
    fn constructor(&self) -> &'static str {
        "note"
    }
    fn sort(&self) -> &'static str {
        "note"
    }
}
impl<M: AstInfo> AstNode<M> for Identifier<M> {
    fn ast_info(&self) -> &M {
        let Self(meta, ..) = self;
        meta
    }
    fn constructor(&self) -> &'static str {
        "identifier"
    }
    fn sort(&self) -> &'static str {
        "identifier"
    }
}
impl<M: AstInfo> AstNode<M> for RegisterName<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::Br(meta, ..) => meta,
            Self::Bt(meta, ..) => meta,
            Self::Pc(meta, ..) => meta,
            Self::Sp(meta, ..) => meta,
            Self::Ir(meta, ..) => meta,
            Self::Oc(meta, ..) => meta,
            Self::Accidental(meta, ..) => meta,
            Self::Num(meta, ..) => meta,
            Self::Note(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::Br(..) => "br",
            Self::Bt(..) => "bt",
            Self::Pc(..) => "pc",
            Self::Sp(..) => "sp",
            Self::Ir(..) => "ir",
            Self::Oc(..) => "oc",
            Self::Accidental(..) => "accidental",
            Self::Num(..) => "num",
            Self::Note(..) => "note",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "register-name"
    }
}
impl<M: AstInfo> AstNode<M> for Instrument<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::Custom(meta, ..) => meta,
            Self::Piano(meta, ..) => meta,
            Self::Celesta(meta, ..) => meta,
            Self::Glockenspiel(meta, ..) => meta,
            Self::Musicbox(meta, ..) => meta,
            Self::Marimba(meta, ..) => meta,
            Self::Dulcimer(meta, ..) => meta,
            Self::Organ(meta, ..) => meta,
            Self::Accordion(meta, ..) => meta,
            Self::Harmonica(meta, ..) => meta,
            Self::Nylonguitar(meta, ..) => meta,
            Self::Steelguitar(meta, ..) => meta,
            Self::Distortionguitar(meta, ..) => meta,
            Self::Acousticbass(meta, ..) => meta,
            Self::Slapbass(meta, ..) => meta,
            Self::Violin(meta, ..) => meta,
            Self::Harp(meta, ..) => meta,
            Self::Timpani(meta, ..) => meta,
            Self::Strings(meta, ..) => meta,
            Self::Synthstrings(meta, ..) => meta,
            Self::Voiceoohs(meta, ..) => meta,
            Self::Synthvox(meta, ..) => meta,
            Self::Brass(meta, ..) => meta,
            Self::Altosax(meta, ..) => meta,
            Self::Tenorsax(meta, ..) => meta,
            Self::Oboe(meta, ..) => meta,
            Self::Enghorn(meta, ..) => meta,
            Self::Flute(meta, ..) => meta,
            Self::Panflute(meta, ..) => meta,
            Self::Whistle(meta, ..) => meta,
            Self::Ocarina(meta, ..) => meta,
            Self::Heavysquarewave(meta, ..) => meta,
            Self::Fantasia(meta, ..) => meta,
            Self::Warmpad(meta, ..) => meta,
            Self::Echodrops(meta, ..) => meta,
            Self::Startheme(meta, ..) => meta,
            Self::Sitar(meta, ..) => meta,
            Self::Banjo(meta, ..) => meta,
            Self::Kalimba(meta, ..) => meta,
            Self::Bagpipe(meta, ..) => meta,
            Self::Fiddle(meta, ..) => meta,
            Self::Steeldrum(meta, ..) => meta,
            Self::Bird(meta, ..) => meta,
            Self::Telephone(meta, ..) => meta,
            Self::Applause(meta, ..) => meta,
            Self::Gunshot(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::Custom(..) => "custom",
            Self::Piano(..) => "piano",
            Self::Celesta(..) => "celesta",
            Self::Glockenspiel(..) => "glockenspiel",
            Self::Musicbox(..) => "musicbox",
            Self::Marimba(..) => "marimba",
            Self::Dulcimer(..) => "dulcimer",
            Self::Organ(..) => "organ",
            Self::Accordion(..) => "accordion",
            Self::Harmonica(..) => "harmonica",
            Self::Nylonguitar(..) => "nylonguitar",
            Self::Steelguitar(..) => "steelguitar",
            Self::Distortionguitar(..) => "distortionguitar",
            Self::Acousticbass(..) => "acousticbass",
            Self::Slapbass(..) => "slapbass",
            Self::Violin(..) => "violin",
            Self::Harp(..) => "harp",
            Self::Timpani(..) => "timpani",
            Self::Strings(..) => "strings",
            Self::Synthstrings(..) => "synthstrings",
            Self::Voiceoohs(..) => "voiceoohs",
            Self::Synthvox(..) => "synthvox",
            Self::Brass(..) => "brass",
            Self::Altosax(..) => "altosax",
            Self::Tenorsax(..) => "tenorsax",
            Self::Oboe(..) => "oboe",
            Self::Enghorn(..) => "enghorn",
            Self::Flute(..) => "flute",
            Self::Panflute(..) => "panflute",
            Self::Whistle(..) => "whistle",
            Self::Ocarina(..) => "ocarina",
            Self::Heavysquarewave(..) => "heavysquarewave",
            Self::Fantasia(..) => "fantasia",
            Self::Warmpad(..) => "warmpad",
            Self::Echodrops(..) => "echodrops",
            Self::Startheme(..) => "startheme",
            Self::Sitar(..) => "sitar",
            Self::Banjo(..) => "banjo",
            Self::Kalimba(..) => "kalimba",
            Self::Bagpipe(..) => "bagpipe",
            Self::Fiddle(..) => "fiddle",
            Self::Steeldrum(..) => "steeldrum",
            Self::Bird(..) => "bird",
            Self::Telephone(..) => "telephone",
            Self::Applause(..) => "applause",
            Self::Gunshot(..) => "gunshot",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "instrument"
    }
}
impl<M: AstInfo> AstNode<M> for Operand<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::LabelRef(meta, ..) => meta,
            Self::Int(meta, ..) => meta,
            Self::Note(meta, ..) => meta,
            Self::Register(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::LabelRef(..) => "label-ref",
            Self::Int(..) => "int",
            Self::Note(..) => "note",
            Self::Register(..) => "register",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "operand"
    }
}
impl<M: AstInfo> AstNode<M> for Program<M> {
    fn ast_info(&self) -> &M {
        let Self(meta, ..) = self;
        meta
    }
    fn constructor(&self) -> &'static str {
        "program"
    }
    fn sort(&self) -> &'static str {
        "program"
    }
}
impl<M: AstInfo> AstNode<M> for Layout<M> {
    fn ast_info(&self) -> &M {
        match self {
            Self::Simple(meta, ..) => meta,
            Self::Comment(meta, ..) => meta,
            _ => unreachable!(),
        }
    }
    fn constructor(&self) -> &'static str {
        match self {
            Self::Simple(..) => "simple",
            Self::Comment(..) => "comment",
            _ => unreachable!(),
        }
    }
    fn sort(&self) -> &'static str {
        "layout"
    }
}