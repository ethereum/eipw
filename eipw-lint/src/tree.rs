/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use comrak::arena_tree::{NodeEdge, Traverse};
use comrak::nodes::{
    Ast, AstNode, NodeCode, NodeCodeBlock, NodeDescriptionItem, NodeHeading, NodeHtmlBlock,
    NodeLink, NodeList, NodeValue, TableAlignment,
};

use std::cell::RefCell;

#[derive(Debug)]
pub enum Next {
    TraverseChildren,
    SkipChildren,
}

pub trait Visitor {
    type Error: std::error::Error;

    fn enter(&mut self, node: &AstNode) -> Result<Next, Self::Error> {
        let data = node.data.borrow();
        match &data.value {
            NodeValue::Document => self.enter_document(&*data),
            NodeValue::FrontMatter(fm) => self.enter_front_matter(&*data, fm),
            NodeValue::BlockQuote => self.enter_block_quote(&*data),
            NodeValue::List(nl) => self.enter_list(&*data, nl),
            NodeValue::Item(nl) => self.enter_item(&*data, nl),
            NodeValue::DescriptionList => self.enter_description_list(&*data),
            NodeValue::DescriptionItem(ndi) => self.enter_description_item(&*data, ndi),
            NodeValue::DescriptionTerm => self.enter_description_term(&*data),
            NodeValue::DescriptionDetails => self.enter_description_details(&*data),
            NodeValue::CodeBlock(ncb) => self.enter_code_block(&*data, ncb),
            NodeValue::HtmlBlock(nhb) => self.enter_html_block(&*data, nhb),
            NodeValue::Paragraph => self.enter_paragraph(&*data),
            NodeValue::Heading(nh) => self.enter_heading(&*data, nh),
            NodeValue::ThematicBreak => self.enter_thematic_break(&*data),
            NodeValue::FootnoteDefinition(fd) => self.enter_footnote_definition(&*data, fd),
            NodeValue::Table(t) => self.enter_table(&*data, t),
            NodeValue::TableRow(r) => self.enter_table_row(&*data, *r),
            NodeValue::TableCell => self.enter_table_cell(&*data),
            NodeValue::Text(txt) => self.enter_text(&*data, txt),
            NodeValue::TaskItem(ti) => self.enter_task_item(&*data, *ti),
            NodeValue::SoftBreak => self.enter_soft_break(&*data),
            NodeValue::LineBreak => self.enter_line_break(&*data),
            NodeValue::Code(nc) => self.enter_code(&*data, nc),
            NodeValue::HtmlInline(html) => self.enter_html_inline(&*data, html),
            NodeValue::Emph => self.enter_emph(&*data),
            NodeValue::Strong => self.enter_strong(&*data),
            NodeValue::Strikethrough => self.enter_strikethrough(&*data),
            NodeValue::Superscript => self.enter_superscript(&*data),
            NodeValue::Link(nl) => self.enter_link(&*data, nl),
            NodeValue::Image(nl) => self.enter_image(&*data, nl),
            NodeValue::FootnoteReference(fr) => self.enter_footnote_reference(&*data, fr),
        }
    }

    fn enter_document(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_front_matter(
        &mut self,
        _ast: &Ast,
        _front_matter: &[u8],
    ) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_block_quote(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_list(&mut self, _ast: &Ast, _node_list: &NodeList) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_item(&mut self, _ast: &Ast, _node_list: &NodeList) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_description_list(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_description_item(
        &mut self,
        _ast: &Ast,
        _description_item: &NodeDescriptionItem,
    ) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_description_term(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_description_details(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_code_block(
        &mut self,
        _ast: &Ast,
        _code_block: &NodeCodeBlock,
    ) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_html_block(
        &mut self,
        _ast: &Ast,
        _html_block: &NodeHtmlBlock,
    ) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_paragraph(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_heading(&mut self, _ast: &Ast, _heading: &NodeHeading) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_thematic_break(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_footnote_definition(
        &mut self,
        _ast: &Ast,
        _footnote_defn: &[u8],
    ) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_table(&mut self, _ast: &Ast, _align: &[TableAlignment]) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_table_row(&mut self, _ast: &Ast, _b: bool) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_table_cell(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_text(&mut self, _ast: &Ast, _txt: &[u8]) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_task_item(&mut self, _ast: &Ast, _checked: bool) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_soft_break(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_line_break(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_code(&mut self, _ast: &Ast, _code: &NodeCode) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_html_inline(&mut self, _ast: &Ast, _html: &[u8]) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_emph(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_strong(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_strikethrough(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_superscript(&mut self, _ast: &Ast) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_link(&mut self, _ast: &Ast, _link: &NodeLink) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_image(&mut self, _ast: &Ast, _link: &NodeLink) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn enter_footnote_reference(&mut self, _ast: &Ast, _refn: &[u8]) -> Result<Next, Self::Error> {
        Ok(Next::TraverseChildren)
    }

    fn depart(&mut self, node: &AstNode) -> Result<(), Self::Error> {
        let data = node.data.borrow();
        match &data.value {
            NodeValue::Document => self.depart_document(&*data),
            NodeValue::FrontMatter(fm) => self.depart_front_matter(&*data, fm),
            NodeValue::BlockQuote => self.depart_block_quote(&*data),
            NodeValue::List(nl) => self.depart_list(&*data, nl),
            NodeValue::Item(nl) => self.depart_item(&*data, nl),
            NodeValue::DescriptionList => self.depart_description_list(&*data),
            NodeValue::DescriptionItem(ndi) => self.depart_description_item(&*data, ndi),
            NodeValue::DescriptionTerm => self.depart_description_term(&*data),
            NodeValue::DescriptionDetails => self.depart_description_details(&*data),
            NodeValue::CodeBlock(ncb) => self.depart_code_block(&*data, ncb),
            NodeValue::HtmlBlock(nhb) => self.depart_html_block(&*data, nhb),
            NodeValue::Paragraph => self.depart_paragraph(&*data),
            NodeValue::Heading(nh) => self.depart_heading(&*data, nh),
            NodeValue::ThematicBreak => self.depart_thematic_break(&*data),
            NodeValue::FootnoteDefinition(fd) => self.depart_footnote_definition(&*data, fd),
            NodeValue::Table(t) => self.depart_table(&*data, t),
            NodeValue::TableRow(r) => self.depart_table_row(&*data, *r),
            NodeValue::TableCell => self.depart_table_cell(&*data),
            NodeValue::Text(txt) => self.depart_text(&*data, txt),
            NodeValue::TaskItem(ti) => self.depart_task_item(&*data, *ti),
            NodeValue::SoftBreak => self.depart_soft_break(&*data),
            NodeValue::LineBreak => self.depart_line_break(&*data),
            NodeValue::Code(nc) => self.depart_code(&*data, nc),
            NodeValue::HtmlInline(html) => self.depart_html_inline(&*data, html),
            NodeValue::Emph => self.depart_emph(&*data),
            NodeValue::Strong => self.depart_strong(&*data),
            NodeValue::Strikethrough => self.depart_strikethrough(&*data),
            NodeValue::Superscript => self.depart_superscript(&*data),
            NodeValue::Link(nl) => self.depart_link(&*data, nl),
            NodeValue::Image(nl) => self.depart_image(&*data, nl),
            NodeValue::FootnoteReference(fr) => self.depart_footnote_reference(&*data, fr),
        }
    }

    fn depart_document(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_front_matter(&mut self, _ast: &Ast, _front_matter: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_block_quote(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_list(&mut self, _ast: &Ast, _node_list: &NodeList) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_item(&mut self, _ast: &Ast, _node_list: &NodeList) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_description_list(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_description_item(
        &mut self,
        _ast: &Ast,
        _description_item: &NodeDescriptionItem,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_description_term(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_description_details(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_code_block(
        &mut self,
        _ast: &Ast,
        _code_block: &NodeCodeBlock,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_html_block(
        &mut self,
        _ast: &Ast,
        _html_block: &NodeHtmlBlock,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_paragraph(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_heading(&mut self, _ast: &Ast, _heading: &NodeHeading) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_thematic_break(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_footnote_definition(
        &mut self,
        _ast: &Ast,
        _footnote_defn: &[u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_table(&mut self, _ast: &Ast, _align: &[TableAlignment]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_table_row(&mut self, _ast: &Ast, _b: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_table_cell(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_text(&mut self, _ast: &Ast, _txt: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_task_item(&mut self, _ast: &Ast, _checked: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_soft_break(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_line_break(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_code(&mut self, _ast: &Ast, _code: &NodeCode) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_html_inline(&mut self, _ast: &Ast, _html: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_emph(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_strong(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_strikethrough(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_superscript(&mut self, _ast: &Ast) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_link(&mut self, _ast: &Ast, _link: &NodeLink) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_image(&mut self, _ast: &Ast, _link: &NodeLink) -> Result<(), Self::Error> {
        Ok(())
    }

    fn depart_footnote_reference(&mut self, _ast: &Ast, _refn: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait TraverseExt {
    fn visit<V>(self, visitor: &mut V) -> Result<(), V::Error>
    where
        V: Visitor;
}

impl<'a> TraverseExt for Traverse<'a, RefCell<Ast>> {
    fn visit<V>(self, visitor: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        let mut skip_until = None;

        for edge in self {
            if let Some(skip) = skip_until {
                if let NodeEdge::End(current) = edge {
                    if current.same_node(skip) {
                        skip_until = None;
                    }
                }
            }

            if skip_until.is_some() {
                continue;
            }

            match edge {
                NodeEdge::End(e) => visitor.depart(e)?,
                NodeEdge::Start(s) => {
                    let next = visitor.enter(s)?;

                    if matches!(next, Next::SkipChildren) {
                        skip_until = Some(s);
                    }
                }
            };
        }

        Ok(())
    }
}
