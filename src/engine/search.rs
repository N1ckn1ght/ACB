use std::cmp::{min, max};
use crate::frame::{util::*, board::Board};
use crate::engine::chara::Chara;

pub fn search(
    chara: &mut Chara,
    board: &mut Board,
    mut alpha: Eval,
    mut beta: Eval,
    maximize: bool,
    depth: i16
) -> Eval {

    


    // IF THE POSITION IS ARLEADY CACHED (and evaluated)
    let hash = *chara.cache_perm_vec.last().unwrap();
    if chara.cache.contains_key(&hash) {
        let mut eval = chara.cache[&hash];
        if depth <= eval.depth {
            
        }
    }

    // IF THE GAME HAS ENDED
    let moves = board.get_legal_moves();
    if moves.len() == 0 {
        if board.is_in_check() {
            let mut mate = 1;
            if !board.turn {
                mate = -1;
            }
            return Eval::new(0.0, mate, board.no);
        }
    }

    // ALPHA/BETA PRUNING
    let mut eval;

    if maximize {
        eval = Eval::new(0.0, -1, depth);
        for mov in moves.into_iter() {
            chara.make_move(board, mov);
            eval = max(eval, search(chara, board, alpha, beta, false, depth, quiet_extension, mov));
            eval.depth += 1;
            chara.revert_move(board);
            alpha = max(alpha, eval);
            if alpha >= beta {
                break;
            }
        }
    } else {
        eval = Eval::new(0.0, 1, depth);
        for mov in moves.into_iter() {
            chara.make_move(board, mov);
            eval = min(eval, search(chara, board, alpha, beta, true, depth, quiet_extension, mov));
            eval.depth += 1;
            chara.revert_move(board);
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
    }

    // cache eval!

    return eval;
}

pub fn extension(
    chara: &mut Chara,
    board: &mut Board,
    mut alpha: Eval,
    mut beta: Eval,
    maximize: bool,
    capture: bool
) -> Eval {
    
    /* EXTENSION CONDITION */

    if !(capture || board.is_in_check()) {
        return chara.eval(board, true);
    }

    let moves = board.get_legal_moves();

    /* GAME END CHECK */

    if moves.len() == 0 {
        if board.is_in_check() {
            let mut score = f32::MAX;
            if board.turn {
                score = f32::MIN;
            }
            return Eval::new(score, 0, true);
        }
    }

    /* PRE-SORTING (captures first) */

    moves.sort();

    /* ALPHA/BETA PRUNING */

    let mut eval;

    if maximize {
        eval = Eval::new(f32::MIN, 0, false);
        for mov in moves.into_iter() {
            chara.make_move(board, mov);
            eval = max(eval, extension(chara, board, alpha, beta, false, mov < CAPTURE_MINIMUM));
            chara.revert_move(board);
            alpha = max(alpha, eval);
            if alpha >= beta {
                break;
            }
        }
    } else {
        eval = Eval::new(f32::MAX, 0, false);
        for mov in moves.into_iter() {
            chara.make_move(board, mov);
            eval = min(eval, search(chara, board, alpha, beta, true, depth, quiet_extension, mov));
            eval.depth += 1;
            chara.revert_move(board);
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
    }

    0
}