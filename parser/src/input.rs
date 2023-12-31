// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    ParseError,
    ParseErrorKind,
    ParseResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Input<'i> {
    offset: usize,
    original_str: &'i str,
    data: &'i str,
}

impl<'i> Input<'i> {
    pub fn new(data: &'i str) -> Self {
        let mut this = Self {
            offset: 0,
            original_str: data,
            data,
        };
        this.trim_start();
        this
    }

    pub const fn original_str(&self) -> &'i str {
        self.original_str
    }

    pub fn consume_until_space(&mut self) -> (&'i str, Range) {
        self.trim_start();
        log::info!("Currently: {:#?}", self);
        self.with_offset_tracking(|this| {
            if let Some(idx) = this.data.find(char::is_whitespace) {
                let ret = &this.data[0..idx];
                this.data = &this.data[idx..];
                ret
            } else {
                let ret = this.data;
                let end = this.data.len();
                this.data = &this.data[end..];
                ret
            }
        })
    }

    pub fn has_next(&self) -> bool {
        !self.data.trim().is_empty()
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub fn trim_start(&mut self) {
        self.with_offset_tracking(|t| {
            t.data = t.data.trim_start();
        });
    }

    pub fn trim(&mut self) {
        self.with_offset_tracking(|t| {
            t.data = t.data.trim();
        });
    }

    pub fn with_offset_tracking<Ret, F: FnOnce(&mut Self) -> Ret>(&mut self, f: F) -> (Ret, Range) {
        let orig_offset = self.offset;

        let len = self.data.len();
        let ret = f(self);
        self.offset += len - self.data.len();

        (ret, Range::new(orig_offset, self.offset))
    }

    pub fn attempt<Ret, F>(&mut self, f: F) -> ParseResult<Ret>
            where F: FnOnce(&mut Self) -> ParseResult<Ret> {
        let restore = *self;
        let ret = f(self);

        if !ret.is_ok() {
            *self = restore;
        }

        ret
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to_error<T>(self, error: ParseErrorKind) -> ParseResult<T> {
        Err(ParseError {
            kind: error,
            range: self
        })
    }
}
