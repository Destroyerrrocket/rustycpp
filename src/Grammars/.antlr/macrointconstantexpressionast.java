// Generated from /home/pol/Documentos/rustycpp/src/grammars/macrointconstantexpressionast.g4 by ANTLR 4.9.2
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class macrointconstantexpressionast extends Parser {
	static { RuntimeMetaData.checkVersion("4.9.2", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		Num=1, LParen=2, RParen=3, Colon=4, Question=5, Tilde=6, Exclamation=7, 
		Plus=8, Minus=9, Star=10, Slash=11, Percent=12, Caret=13, Ampersand=14, 
		Pipe=15, DoubleEqual=16, ExclamationEqual=17, Less=18, Greater=19, LessEqual=20, 
		GreaterEqual=21, Spaceship=22, DoubleAmpersand=23, DoublePipe=24, DoubleLess=25, 
		DoubleGreater=26, DoublePlus=27, DoubleMinus=28, Comma=29, And=30, Or=31, 
		Xor=32, Not=33, Bitand=34, Bitor=35, Compl=36;
	public static final int
		RULE_exprRes = 0, RULE_expr = 1;
	private static String[] makeRuleNames() {
		return new String[] {
			"exprRes", "expr"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "Num", "LParen", "RParen", "Colon", "Question", "Tilde", "Exclamation", 
			"Plus", "Minus", "Star", "Slash", "Percent", "Caret", "Ampersand", "Pipe", 
			"DoubleEqual", "ExclamationEqual", "Less", "Greater", "LessEqual", "GreaterEqual", 
			"Spaceship", "DoubleAmpersand", "DoublePipe", "DoubleLess", "DoubleGreater", 
			"DoublePlus", "DoubleMinus", "Comma", "And", "Or", "Xor", "Not", "Bitand", 
			"Bitor", "Compl"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "macrointconstantexpressionast.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public macrointconstantexpressionast(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	public static class ExprResContext extends ParserRuleContext {
		public ExprResContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_exprRes; }
	 
		public ExprResContext() { }
		public void copyFrom(ExprResContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class ResultContext extends ExprResContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ResultContext(ExprResContext ctx) { copyFrom(ctx); }
	}

	public final ExprResContext exprRes() throws RecognitionException {
		ExprResContext _localctx = new ExprResContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_exprRes);
		try {
			_localctx = new ResultContext(_localctx);
			enterOuterAlt(_localctx, 1);
			{
			setState(4);
			expr(0);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ExprContext extends ParserRuleContext {
		public ExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expr; }
	 
		public ExprContext() { }
		public void copyFrom(ExprContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class SinglePostIncrementContext extends ExprContext {
		public Token op;
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode DoubleMinus() { return getToken(macrointconstantexpressionast.DoubleMinus, 0); }
		public TerminalNode DoublePlus() { return getToken(macrointconstantexpressionast.DoublePlus, 0); }
		public SinglePostIncrementContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class UnarySignContext extends ExprContext {
		public Token op;
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode Minus() { return getToken(macrointconstantexpressionast.Minus, 0); }
		public TerminalNode Plus() { return getToken(macrointconstantexpressionast.Plus, 0); }
		public UnarySignContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class BitOrContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Pipe() { return getToken(macrointconstantexpressionast.Pipe, 0); }
		public TerminalNode Or() { return getToken(macrointconstantexpressionast.Or, 0); }
		public BitOrContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class AddSubContext extends ExprContext {
		public Token op;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Plus() { return getToken(macrointconstantexpressionast.Plus, 0); }
		public TerminalNode Minus() { return getToken(macrointconstantexpressionast.Minus, 0); }
		public AddSubContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class BitShiftContext extends ExprContext {
		public Token op;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode DoubleLess() { return getToken(macrointconstantexpressionast.DoubleLess, 0); }
		public TerminalNode DoubleGreater() { return getToken(macrointconstantexpressionast.DoubleGreater, 0); }
		public BitShiftContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class LogOrContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode DoublePipe() { return getToken(macrointconstantexpressionast.DoublePipe, 0); }
		public TerminalNode Or() { return getToken(macrointconstantexpressionast.Or, 0); }
		public LogOrContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class TernaryContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Question() { return getToken(macrointconstantexpressionast.Question, 0); }
		public TerminalNode Colon() { return getToken(macrointconstantexpressionast.Colon, 0); }
		public TernaryContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class SpaceshipContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Spaceship() { return getToken(macrointconstantexpressionast.Spaceship, 0); }
		public SpaceshipContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class SinglePreIncrementContext extends ExprContext {
		public Token op;
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode DoubleMinus() { return getToken(macrointconstantexpressionast.DoubleMinus, 0); }
		public TerminalNode DoublePlus() { return getToken(macrointconstantexpressionast.DoublePlus, 0); }
		public SinglePreIncrementContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class LogAndContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode DoubleAmpersand() { return getToken(macrointconstantexpressionast.DoubleAmpersand, 0); }
		public TerminalNode And() { return getToken(macrointconstantexpressionast.And, 0); }
		public LogAndContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class CommaContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Comma() { return getToken(macrointconstantexpressionast.Comma, 0); }
		public CommaContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class MulDivModContext extends ExprContext {
		public Token op;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Star() { return getToken(macrointconstantexpressionast.Star, 0); }
		public TerminalNode Slash() { return getToken(macrointconstantexpressionast.Slash, 0); }
		public TerminalNode Percent() { return getToken(macrointconstantexpressionast.Percent, 0); }
		public MulDivModContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class BitXorContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Caret() { return getToken(macrointconstantexpressionast.Caret, 0); }
		public TerminalNode Xor() { return getToken(macrointconstantexpressionast.Xor, 0); }
		public BitXorContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class LogicalOrBitNotContext extends ExprContext {
		public Token op;
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode Exclamation() { return getToken(macrointconstantexpressionast.Exclamation, 0); }
		public TerminalNode Not() { return getToken(macrointconstantexpressionast.Not, 0); }
		public TerminalNode Tilde() { return getToken(macrointconstantexpressionast.Tilde, 0); }
		public TerminalNode Compl() { return getToken(macrointconstantexpressionast.Compl, 0); }
		public LogicalOrBitNotContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class NumberContext extends ExprContext {
		public TerminalNode Num() { return getToken(macrointconstantexpressionast.Num, 0); }
		public NumberContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class EqualitiesContext extends ExprContext {
		public Token op;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode DoubleEqual() { return getToken(macrointconstantexpressionast.DoubleEqual, 0); }
		public TerminalNode ExclamationEqual() { return getToken(macrointconstantexpressionast.ExclamationEqual, 0); }
		public EqualitiesContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class BitAndContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Ampersand() { return getToken(macrointconstantexpressionast.Ampersand, 0); }
		public TerminalNode Bitand() { return getToken(macrointconstantexpressionast.Bitand, 0); }
		public BitAndContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class CompareContext extends ExprContext {
		public Token op;
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode Less() { return getToken(macrointconstantexpressionast.Less, 0); }
		public TerminalNode LessEqual() { return getToken(macrointconstantexpressionast.LessEqual, 0); }
		public TerminalNode Greater() { return getToken(macrointconstantexpressionast.Greater, 0); }
		public TerminalNode GreaterEqual() { return getToken(macrointconstantexpressionast.GreaterEqual, 0); }
		public CompareContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class ParenContext extends ExprContext {
		public TerminalNode LParen() { return getToken(macrointconstantexpressionast.LParen, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode RParen() { return getToken(macrointconstantexpressionast.RParen, 0); }
		public ParenContext(ExprContext ctx) { copyFrom(ctx); }
	}

	public final ExprContext expr() throws RecognitionException {
		return expr(0);
	}

	private ExprContext expr(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		ExprContext _localctx = new ExprContext(_ctx, _parentState);
		ExprContext _prevctx = _localctx;
		int _startState = 2;
		enterRecursionRule(_localctx, 2, RULE_expr, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(18);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case LParen:
				{
				_localctx = new ParenContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(7);
				match(LParen);
				setState(8);
				expr(0);
				setState(9);
				match(RParen);
				}
				break;
			case DoublePlus:
			case DoubleMinus:
				{
				_localctx = new SinglePreIncrementContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(11);
				((SinglePreIncrementContext)_localctx).op = _input.LT(1);
				_la = _input.LA(1);
				if ( !(_la==DoublePlus || _la==DoubleMinus) ) {
					((SinglePreIncrementContext)_localctx).op = (Token)_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(12);
				expr(17);
				}
				break;
			case Plus:
			case Minus:
				{
				_localctx = new UnarySignContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(13);
				((UnarySignContext)_localctx).op = _input.LT(1);
				_la = _input.LA(1);
				if ( !(_la==Plus || _la==Minus) ) {
					((UnarySignContext)_localctx).op = (Token)_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(14);
				expr(16);
				}
				break;
			case Tilde:
			case Exclamation:
			case Not:
			case Compl:
				{
				_localctx = new LogicalOrBitNotContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(15);
				((LogicalOrBitNotContext)_localctx).op = _input.LT(1);
				_la = _input.LA(1);
				if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << Tilde) | (1L << Exclamation) | (1L << Not) | (1L << Compl))) != 0)) ) {
					((LogicalOrBitNotContext)_localctx).op = (Token)_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(16);
				expr(15);
				}
				break;
			case Num:
				{
				_localctx = new NumberContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(17);
				match(Num);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(66);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(64);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,1,_ctx) ) {
					case 1:
						{
						_localctx = new MulDivModContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(20);
						if (!(precpred(_ctx, 14))) throw new FailedPredicateException(this, "precpred(_ctx, 14)");
						setState(21);
						((MulDivModContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << Star) | (1L << Slash) | (1L << Percent))) != 0)) ) {
							((MulDivModContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(22);
						expr(15);
						}
						break;
					case 2:
						{
						_localctx = new AddSubContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(23);
						if (!(precpred(_ctx, 13))) throw new FailedPredicateException(this, "precpred(_ctx, 13)");
						setState(24);
						((AddSubContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==Plus || _la==Minus) ) {
							((AddSubContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(25);
						expr(14);
						}
						break;
					case 3:
						{
						_localctx = new BitShiftContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(26);
						if (!(precpred(_ctx, 12))) throw new FailedPredicateException(this, "precpred(_ctx, 12)");
						setState(27);
						((BitShiftContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==DoubleLess || _la==DoubleGreater) ) {
							((BitShiftContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(28);
						expr(13);
						}
						break;
					case 4:
						{
						_localctx = new SpaceshipContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(29);
						if (!(precpred(_ctx, 11))) throw new FailedPredicateException(this, "precpred(_ctx, 11)");
						setState(30);
						match(Spaceship);
						setState(31);
						expr(12);
						}
						break;
					case 5:
						{
						_localctx = new CompareContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(32);
						if (!(precpred(_ctx, 10))) throw new FailedPredicateException(this, "precpred(_ctx, 10)");
						setState(33);
						((CompareContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << Less) | (1L << Greater) | (1L << LessEqual) | (1L << GreaterEqual))) != 0)) ) {
							((CompareContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(34);
						expr(11);
						}
						break;
					case 6:
						{
						_localctx = new EqualitiesContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(35);
						if (!(precpred(_ctx, 9))) throw new FailedPredicateException(this, "precpred(_ctx, 9)");
						setState(36);
						((EqualitiesContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==DoubleEqual || _la==ExclamationEqual) ) {
							((EqualitiesContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(37);
						expr(10);
						}
						break;
					case 7:
						{
						_localctx = new BitAndContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(38);
						if (!(precpred(_ctx, 8))) throw new FailedPredicateException(this, "precpred(_ctx, 8)");
						setState(39);
						_la = _input.LA(1);
						if ( !(_la==Ampersand || _la==Bitand) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(40);
						expr(9);
						}
						break;
					case 8:
						{
						_localctx = new BitXorContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(41);
						if (!(precpred(_ctx, 7))) throw new FailedPredicateException(this, "precpred(_ctx, 7)");
						setState(42);
						_la = _input.LA(1);
						if ( !(_la==Caret || _la==Xor) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(43);
						expr(8);
						}
						break;
					case 9:
						{
						_localctx = new BitOrContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(44);
						if (!(precpred(_ctx, 6))) throw new FailedPredicateException(this, "precpred(_ctx, 6)");
						setState(45);
						_la = _input.LA(1);
						if ( !(_la==Pipe || _la==Or) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(46);
						expr(7);
						}
						break;
					case 10:
						{
						_localctx = new LogAndContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(47);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(48);
						_la = _input.LA(1);
						if ( !(_la==DoubleAmpersand || _la==And) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(49);
						expr(6);
						}
						break;
					case 11:
						{
						_localctx = new LogOrContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(50);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(51);
						_la = _input.LA(1);
						if ( !(_la==DoublePipe || _la==Or) ) {
						_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						setState(52);
						expr(5);
						}
						break;
					case 12:
						{
						_localctx = new TernaryContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(53);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(54);
						match(Question);
						setState(55);
						expr(0);
						setState(56);
						match(Colon);
						setState(57);
						expr(3);
						}
						break;
					case 13:
						{
						_localctx = new CommaContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(59);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(60);
						match(Comma);
						setState(61);
						expr(3);
						}
						break;
					case 14:
						{
						_localctx = new SinglePostIncrementContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(62);
						if (!(precpred(_ctx, 18))) throw new FailedPredicateException(this, "precpred(_ctx, 18)");
						setState(63);
						((SinglePostIncrementContext)_localctx).op = _input.LT(1);
						_la = _input.LA(1);
						if ( !(_la==DoublePlus || _la==DoubleMinus) ) {
							((SinglePostIncrementContext)_localctx).op = (Token)_errHandler.recoverInline(this);
						}
						else {
							if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
							_errHandler.reportMatch(this);
							consume();
						}
						}
						break;
					}
					} 
				}
				setState(68);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 1:
			return expr_sempred((ExprContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean expr_sempred(ExprContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 14);
		case 1:
			return precpred(_ctx, 13);
		case 2:
			return precpred(_ctx, 12);
		case 3:
			return precpred(_ctx, 11);
		case 4:
			return precpred(_ctx, 10);
		case 5:
			return precpred(_ctx, 9);
		case 6:
			return precpred(_ctx, 8);
		case 7:
			return precpred(_ctx, 7);
		case 8:
			return precpred(_ctx, 6);
		case 9:
			return precpred(_ctx, 5);
		case 10:
			return precpred(_ctx, 4);
		case 11:
			return precpred(_ctx, 3);
		case 12:
			return precpred(_ctx, 2);
		case 13:
			return precpred(_ctx, 18);
		}
		return true;
	}

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3&H\4\2\t\2\4\3\t\3"+
		"\3\2\3\2\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\5\3\25\n\3\3"+
		"\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3"+
		"\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3"+
		"\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\7\3C\n\3\f\3\16\3F\13\3\3\3\2\3\4\4"+
		"\2\4\2\16\3\2\35\36\3\2\n\13\5\2\b\t##&&\3\2\f\16\3\2\33\34\3\2\24\27"+
		"\3\2\22\23\4\2\20\20$$\4\2\17\17\"\"\4\2\21\21!!\4\2\31\31  \4\2\32\32"+
		"!!\2W\2\6\3\2\2\2\4\24\3\2\2\2\6\7\5\4\3\2\7\3\3\2\2\2\b\t\b\3\1\2\t\n"+
		"\7\4\2\2\n\13\5\4\3\2\13\f\7\5\2\2\f\25\3\2\2\2\r\16\t\2\2\2\16\25\5\4"+
		"\3\23\17\20\t\3\2\2\20\25\5\4\3\22\21\22\t\4\2\2\22\25\5\4\3\21\23\25"+
		"\7\3\2\2\24\b\3\2\2\2\24\r\3\2\2\2\24\17\3\2\2\2\24\21\3\2\2\2\24\23\3"+
		"\2\2\2\25D\3\2\2\2\26\27\f\20\2\2\27\30\t\5\2\2\30C\5\4\3\21\31\32\f\17"+
		"\2\2\32\33\t\3\2\2\33C\5\4\3\20\34\35\f\16\2\2\35\36\t\6\2\2\36C\5\4\3"+
		"\17\37 \f\r\2\2 !\7\30\2\2!C\5\4\3\16\"#\f\f\2\2#$\t\7\2\2$C\5\4\3\r%"+
		"&\f\13\2\2&\'\t\b\2\2\'C\5\4\3\f()\f\n\2\2)*\t\t\2\2*C\5\4\3\13+,\f\t"+
		"\2\2,-\t\n\2\2-C\5\4\3\n./\f\b\2\2/\60\t\13\2\2\60C\5\4\3\t\61\62\f\7"+
		"\2\2\62\63\t\f\2\2\63C\5\4\3\b\64\65\f\6\2\2\65\66\t\r\2\2\66C\5\4\3\7"+
		"\678\f\5\2\289\7\7\2\29:\5\4\3\2:;\7\6\2\2;<\5\4\3\5<C\3\2\2\2=>\f\4\2"+
		"\2>?\7\37\2\2?C\5\4\3\5@A\f\24\2\2AC\t\2\2\2B\26\3\2\2\2B\31\3\2\2\2B"+
		"\34\3\2\2\2B\37\3\2\2\2B\"\3\2\2\2B%\3\2\2\2B(\3\2\2\2B+\3\2\2\2B.\3\2"+
		"\2\2B\61\3\2\2\2B\64\3\2\2\2B\67\3\2\2\2B=\3\2\2\2B@\3\2\2\2CF\3\2\2\2"+
		"DB\3\2\2\2DE\3\2\2\2E\5\3\2\2\2FD\3\2\2\2\5\24BD";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}