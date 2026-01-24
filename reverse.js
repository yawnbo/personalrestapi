/*** BEGIN FILE: /var/www/lookmovie.ag/v3.9.36/web/assets/330939e7/js/main/player/Player.js.tmp ***/
var $jscomp = $jscomp ||
{
};
$jscomp.scope = {};
$jscomp.createTemplateTagFirstArg = function (a) {
  return a.raw = a
};
$jscomp.createTemplateTagFirstArgWithRaw = function (a, b) {
  a.raw = b;
  return a
};
$jscomp.arrayIteratorImpl = function (a) {
  var b = 0;
  return function () {
    return b < a.length ? {
      done: !1,
      value: a[b++]
    }
     : {
      done: !0
    }
  }
};
$jscomp.arrayIterator = function (a) {
  return {
    next: $jscomp.arrayIteratorImpl(a)
  }
};
$jscomp.makeIterator = function (a) {
  var b = 'undefined' != typeof Symbol &&
  Symbol.iterator &&
  a[Symbol.iterator];
  return b ? b.call(a) : $jscomp.arrayIterator(a)
};
$jscomp.arrayFromIterator = function (a) {
  for (var b, e = []; !(b = a.next()).done; ) e.push(b.value);
  return e
};
$jscomp.arrayFromIterable = function (a) {
  return a instanceof Array ? a : $jscomp.arrayFromIterator($jscomp.makeIterator(a))
};
$jscomp.ASSUME_ES5 = !1;
$jscomp.ASSUME_NO_NATIVE_MAP = !1;
$jscomp.ASSUME_NO_NATIVE_SET = !1;
$jscomp.SIMPLE_FROUND_POLYFILL = !1;
$jscomp.ISOLATE_POLYFILLS = !1;
$jscomp.FORCE_POLYFILL_PROMISE = !1;
$jscomp.FORCE_POLYFILL_PROMISE_WHEN_NO_UNHANDLED_REJECTION = !1;
$jscomp.defineProperty = $jscomp.ASSUME_ES5 ||
'function' == typeof Object.defineProperties ? Object.defineProperty : function (a, b, e) {
  if (a == Array.prototype || a == Object.prototype) return a;
  a[b] = e.value;
  return a
};
$jscomp.getGlobal = function (a) {
  a = [
    'object' == typeof globalThis &&
    globalThis,
    a,
    'object' == typeof window &&
    window,
    'object' == typeof self &&
    self,
    'object' == typeof global &&
    global
  ];
  for (var b = 0; b < a.length; ++b) {
    var e = a[b];
    if (e && e.Math == Math) return e
  }
  throw Error('Cannot find global object');
};
$jscomp.global = $jscomp.getGlobal(this);
$jscomp.IS_SYMBOL_NATIVE = 'function' === typeof Symbol &&
'symbol' === typeof Symbol('x');
$jscomp.TRUST_ES6_POLYFILLS = !$jscomp.ISOLATE_POLYFILLS ||
$jscomp.IS_SYMBOL_NATIVE;
$jscomp.polyfills = {};
$jscomp.propertyToPolyfillSymbol = {};
$jscomp.POLYFILL_PREFIX = '$jscp$';
var $jscomp$lookupPolyfilledValue = function (a, b) {
  var e = $jscomp.propertyToPolyfillSymbol[b];
  if (null == e) return a[b];
  e = a[e];
  return void 0 !== e ? e : a[b]
};
$jscomp.polyfill = function (a, b, e, m) {
  b &&
  (
    $jscomp.ISOLATE_POLYFILLS ? $jscomp.polyfillIsolated(a, b, e, m) : $jscomp.polyfillUnisolated(a, b, e, m)
  )
};
$jscomp.polyfillUnisolated = function (a, b, e, m) {
  e = $jscomp.global;
  a = a.split('.');
  for (m = 0; m < a.length - 1; m++) {
    var f = a[m];
    if (!(f in e)) return;
    e = e[f]
  }
  a = a[a.length - 1];
  m = e[a];
  b = b(m);
  b != m &&
  null != b &&
  $jscomp.defineProperty(e, a, {
    configurable: !0,
    writable: !0,
    value: b
  })
};
$jscomp.polyfillIsolated = function (a, b, e, m) {
  var f = a.split('.');
  a = 1 === f.length;
  m = f[0];
  m = !a &&
  m in $jscomp.polyfills ? $jscomp.polyfills : $jscomp.global;
  for (var d = 0; d < f.length - 1; d++) {
    var c = f[d];
    if (!(c in m)) return;
    m = m[c]
  }
  f = f[f.length - 1];
  e = $jscomp.IS_SYMBOL_NATIVE &&
  'es6' === e ? m[f] : null;
  b = b(e);
  null != b &&
  (
    a ? $jscomp.defineProperty($jscomp.polyfills, f, {
      configurable: !0,
      writable: !0,
      value: b
    }) : b !== e &&
    (
      $jscomp.propertyToPolyfillSymbol[f] = $jscomp.IS_SYMBOL_NATIVE ? $jscomp.global.Symbol(f) : $jscomp.POLYFILL_PREFIX + f,
      f = $jscomp.propertyToPolyfillSymbol[f],
      $jscomp.defineProperty(m, f, {
        configurable: !0,
        writable: !0,
        value: b
      })
    )
  )
};
$jscomp.underscoreProtoCanBeSet = function () {
  var a = {
    a: !0
  },
  b = {};
  try {
    return b.__proto__ = a,
    b.a
  } catch (e) {
  }
  return !1
};
$jscomp.setPrototypeOf = $jscomp.TRUST_ES6_POLYFILLS &&
'function' == typeof Object.setPrototypeOf ? Object.setPrototypeOf : $jscomp.underscoreProtoCanBeSet() ? function (a, b) {
  a.__proto__ = b;
  if (a.__proto__ !== b) throw new TypeError(a + ' is not extensible');
  return a
}
 : null;
$jscomp.generator = {};
$jscomp.generator.ensureIteratorResultIsObject_ = function (a) {
  if (!(a instanceof Object)) throw new TypeError('Iterator result ' + a + ' is not an object');
};
$jscomp.generator.Context = function () {
  this.isRunning_ = !1;
  this.yieldAllIterator_ = null;
  this.yieldResult = void 0;
  this.nextAddress = 1;
  this.finallyAddress_ = this.catchAddress_ = 0;
  this.finallyContexts_ = this.abruptCompletion_ = null
};
$jscomp.generator.Context.prototype.start_ = function () {
  if (this.isRunning_) throw new TypeError('Generator is already running');
  this.isRunning_ = !0
};
$jscomp.generator.Context.prototype.stop_ = function () {
  this.isRunning_ = !1
};
$jscomp.generator.Context.prototype.jumpToErrorHandler_ = function () {
  this.nextAddress = this.catchAddress_ ||
  this.finallyAddress_
};
$jscomp.generator.Context.prototype.next_ = function (a) {
  this.yieldResult = a
};
$jscomp.generator.Context.prototype.throw_ = function (a) {
  this.abruptCompletion_ = {
    exception: a,
    isException: !0
  };
  this.jumpToErrorHandler_()
};
$jscomp.generator.Context.prototype.return = function (a) {
  this.abruptCompletion_ = {
    return : a
  };
  this.nextAddress = this.finallyAddress_
};
$jscomp.generator.Context.prototype.jumpThroughFinallyBlocks = function (a) {
  this.abruptCompletion_ = {
    jumpTo: a
  };
  this.nextAddress = this.finallyAddress_
};
$jscomp.generator.Context.prototype.yield = function (a, b) {
  this.nextAddress = b;
  return {
    value: a
  }
};
$jscomp.generator.Context.prototype.yieldAll = function (a, b) {
  a = $jscomp.makeIterator(a);
  var e = a.next();
  $jscomp.generator.ensureIteratorResultIsObject_(e);
  if (e.done) this.yieldResult = e.value,
  this.nextAddress = b;
   else return this.yieldAllIterator_ = a,
  this.yield(e.value, b)
};
$jscomp.generator.Context.prototype.jumpTo = function (a) {
  this.nextAddress = a
};
$jscomp.generator.Context.prototype.jumpToEnd = function () {
  this.nextAddress = 0
};
$jscomp.generator.Context.prototype.setCatchFinallyBlocks = function (a, b) {
  this.catchAddress_ = a;
  void 0 != b &&
  (this.finallyAddress_ = b)
};
$jscomp.generator.Context.prototype.setFinallyBlock = function (a) {
  this.catchAddress_ = 0;
  this.finallyAddress_ = a ||
  0
};
$jscomp.generator.Context.prototype.leaveTryBlock = function (a, b) {
  this.nextAddress = a;
  this.catchAddress_ = b ||
  0
};
$jscomp.generator.Context.prototype.enterCatchBlock = function (a) {
  this.catchAddress_ = a ||
  0;
  a = this.abruptCompletion_.exception;
  this.abruptCompletion_ = null;
  return a
};
$jscomp.generator.Context.prototype.enterFinallyBlock = function (a, b, e) {
  e ? this.finallyContexts_[e] = this.abruptCompletion_ : this.finallyContexts_ = [
    this.abruptCompletion_
  ];
  this.catchAddress_ = a ||
  0;
  this.finallyAddress_ = b ||
  0
};
$jscomp.generator.Context.prototype.leaveFinallyBlock = function (a, b) {
  b = this.finallyContexts_.splice(b || 0) [0];
  if (b = this.abruptCompletion_ = this.abruptCompletion_ || b) {
    if (b.isException) return this.jumpToErrorHandler_();
    void 0 != b.jumpTo &&
    this.finallyAddress_ < b.jumpTo ? (this.nextAddress = b.jumpTo, this.abruptCompletion_ = null) : this.nextAddress = this.finallyAddress_
  } else this.nextAddress = a
};
$jscomp.generator.Context.prototype.forIn = function (a) {
  return new $jscomp.generator.Context.PropertyIterator(a)
};
$jscomp.generator.Context.PropertyIterator = function (a) {
  this.object_ = a;
  this.properties_ = [];
  for (var b in a) this.properties_.push(b);
  this.properties_.reverse()
};
$jscomp.generator.Context.PropertyIterator.prototype.getNext = function () {
  for (; 0 < this.properties_.length; ) {
    var a = this.properties_.pop();
    if (a in this.object_) return a
  }
  return null
};
$jscomp.generator.Engine_ = function (a) {
  this.context_ = new $jscomp.generator.Context;
  this.program_ = a
};
$jscomp.generator.Engine_.prototype.next_ = function (a) {
  this.context_.start_();
  if (this.context_.yieldAllIterator_) return this.yieldAllStep_(this.context_.yieldAllIterator_.next, a, this.context_.next_);
  this.context_.next_(a);
  return this.nextStep_()
};
$jscomp.generator.Engine_.prototype.return_ = function (a) {
  this.context_.start_();
  var b = this.context_.yieldAllIterator_;
  if (b) return this.yieldAllStep_(
    'return' in b ? b['return'] : function (e) {
      return {
        value: e,
        done: !0
      }
    },
    a,
    this.context_.return
  );
  this.context_.return(a);
  return this.nextStep_()
};
$jscomp.generator.Engine_.prototype.throw_ = function (a) {
  this.context_.start_();
  if (this.context_.yieldAllIterator_) return this.yieldAllStep_(
    this.context_.yieldAllIterator_['throw'],
    a,
    this.context_.next_
  );
  this.context_.throw_(a);
  return this.nextStep_()
};
$jscomp.generator.Engine_.prototype.yieldAllStep_ = function (a, b, e) {
  try {
    var m = a.call(this.context_.yieldAllIterator_, b);
    $jscomp.generator.ensureIteratorResultIsObject_(m);
    if (!m.done) return this.context_.stop_(),
    m;
    var f = m.value
  } catch (d) {
    return this.context_.yieldAllIterator_ = null,
    this.context_.throw_(d),
    this.nextStep_()
  }
  this.context_.yieldAllIterator_ = null;
  e.call(this.context_, f);
  return this.nextStep_()
};
$jscomp.generator.Engine_.prototype.nextStep_ = function () {
  for (; this.context_.nextAddress; ) try {
    var a = this.program_(this.context_);
    if (a) return this.context_.stop_(),
    {
      value: a.value,
      done: !1
    }
  } catch (b) {
    this.context_.yieldResult = void 0,
    this.context_.throw_(b)
  }
  this.context_.stop_();
  if (this.context_.abruptCompletion_) {
    a = this.context_.abruptCompletion_;
    this.context_.abruptCompletion_ = null;
    if (a.isException) throw a.exception;
    return {
      value: a.return,
      done: !0
    }
  }
  return {
    value: void 0,
    done: !0
  }
};
$jscomp.generator.Generator_ = function (a) {
  this.next = function (b) {
    return a.next_(b)
  };
  this.throw = function (b) {
    return a.throw_(b)
  };
  this.return = function (b) {
    return a.return_(b)
  };
  this[Symbol.iterator] = function () {
    return this
  }
};
$jscomp.generator.createGenerator = function (a, b) {
  b = new $jscomp.generator.Generator_(new $jscomp.generator.Engine_(b));
  $jscomp.setPrototypeOf &&
  a.prototype &&
  $jscomp.setPrototypeOf(b, a.prototype);
  return b
};
$jscomp.asyncExecutePromiseGenerator = function (a) {
  function b(m) {
    return a.next(m)
  }
  function e(m) {
    return a.throw(m)
  }
  return new Promise(
    function (m, f) {
      function d(c) {
        c.done ? m(c.value) : Promise.resolve(c.value).then(b, e).then(d, f)
      }
      d(a.next())
    }
  )
};
$jscomp.asyncExecutePromiseGeneratorFunction = function (a) {
  return $jscomp.asyncExecutePromiseGenerator(a())
};
$jscomp.asyncExecutePromiseGeneratorProgram = function (a) {
  return $jscomp.asyncExecutePromiseGenerator(
    new $jscomp.generator.Generator_(new $jscomp.generator.Engine_(a))
  )
};
$jscomp.initSymbol = function () {
};
$jscomp.polyfill(
  'Symbol',
  function (a) {
    if (a) return a;
    var b = function (f, d) {
      this.$jscomp$symbol$id_ = f;
      $jscomp.defineProperty(this, 'description', {
        configurable: !0,
        writable: !0,
        value: d
      })
    };
    b.prototype.toString = function () {
      return this.$jscomp$symbol$id_
    };
    var e = 0,
    m = function (f) {
      if (this instanceof m) throw new TypeError('Symbol is not a constructor');
      return new b('jscomp_symbol_' + (f || '') + '_' + e++, f)
    };
    return m
  },
  'es6',
  'es3'
);
$jscomp.polyfill(
  'Symbol.iterator',
  function (a) {
    if (a) return a;
    a = Symbol('Symbol.iterator');
    for (
      var b = 'Array Int8Array Uint8Array Uint8ClampedArray Int16Array Uint16Array Int32Array Uint32Array Float32Array Float64Array'.split(' '),
      e = 0;
      e < b.length;
      e++
    ) {
      var m = $jscomp.global[b[e]];
      'function' === typeof m &&
      'function' != typeof m.prototype[a] &&
      $jscomp.defineProperty(
        m.prototype,
        a,
        {
          configurable: !0,
          writable: !0,
          value: function () {
            return $jscomp.iteratorPrototype($jscomp.arrayIteratorImpl(this))
          }
        }
      )
    }
    return a
  },
  'es6',
  'es3'
);
$jscomp.iteratorPrototype = function (a) {
  a = {
    next: a
  };
  a[Symbol.iterator] = function () {
    return this
  };
  return a
};
$jscomp.polyfill(
  'Promise',
  function (a) {
    function b() {
      this.batch_ = null
    }
    function e(c) {
      return c instanceof f ? c : new f(function (g, h) {
        g(c)
      })
    }
    if (
      a &&
      (
        !(
          $jscomp.FORCE_POLYFILL_PROMISE ||
          $jscomp.FORCE_POLYFILL_PROMISE_WHEN_NO_UNHANDLED_REJECTION &&
          'undefined' === typeof $jscomp.global.PromiseRejectionEvent
        ) ||
        !$jscomp.global.Promise ||
        - 1 === $jscomp.global.Promise.toString().indexOf('[native code]')
      )
    ) return a;
    b.prototype.asyncExecute = function (c) {
      if (null == this.batch_) {
        this.batch_ = [];
        var g = this;
        this.asyncExecuteFunction(function () {
          g.executeBatch_()
        })
      }
      this.batch_.push(c)
    };
    var m = $jscomp.global.setTimeout;
    b.prototype.asyncExecuteFunction = function (c) {
      m(c, 0)
    };
    b.prototype.executeBatch_ = function () {
      for (; this.batch_ && this.batch_.length; ) {
        var c = this.batch_;
        this.batch_ = [];
        for (var g = 0; g < c.length; ++g) {
          var h = c[g];
          c[g] = null;
          try {
            h()
          } catch (l) {
            this.asyncThrow_(l)
          }
        }
      }
      this.batch_ = null
    };
    b.prototype.asyncThrow_ = function (c) {
      this.asyncExecuteFunction(function () {
        throw c;
      })
    };
    var f = function (c) {
      this.state_ = 0;
      this.result_ = void 0;
      this.onSettledCallbacks_ = [];
      this.isRejectionHandled_ = !1;
      var g = this.createResolveAndReject_();
      try {
        c(g.resolve, g.reject)
      } catch (h) {
        g.reject(h)
      }
    };
    f.prototype.createResolveAndReject_ = function () {
      function c(l) {
        return function (k) {
          h ||
          (h = !0, l.call(g, k))
        }
      }
      var g = this,
      h = !1;
      return {
        resolve: c(this.resolveTo_),
        reject: c(this.reject_)
      }
    };
    f.prototype.resolveTo_ = function (c) {
      if (c === this) this.reject_(new TypeError('A Promise cannot resolve to itself'));
       else if (c instanceof f) this.settleSameAsPromise_(c);
       else {
        a: switch (typeof c) {
          case 'object':
            var g = null != c;
            break a;
          case 'function':
            g = !0;
            break a;
          default:
            g = !1
        }
        g ? this.resolveToNonPromiseObj_(c) :
        this.fulfill_(c)
      }
    };
    f.prototype.resolveToNonPromiseObj_ = function (c) {
      var g = void 0;
      try {
        g = c.then
      } catch (h) {
        this.reject_(h);
        return
      }
      'function' == typeof g ? this.settleSameAsThenable_(g, c) : this.fulfill_(c)
    };
    f.prototype.reject_ = function (c) {
      this.settle_(2, c)
    };
    f.prototype.fulfill_ = function (c) {
      this.settle_(1, c)
    };
    f.prototype.settle_ = function (c, g) {
      if (0 != this.state_) throw Error(
        'Cannot settle(' + c + ', ' + g + '): Promise already settled in state' + this.state_
      );
      this.state_ = c;
      this.result_ = g;
      2 === this.state_ &&
      this.scheduleUnhandledRejectionCheck_();
      this.executeOnSettledCallbacks_()
    };
    f.prototype.scheduleUnhandledRejectionCheck_ = function () {
      var c = this;
      m(
        function () {
          if (c.notifyUnhandledRejection_()) {
            var g = $jscomp.global.console;
            'undefined' !== typeof g &&
            g.error(c.result_)
          }
        },
        1
      )
    };
    f.prototype.notifyUnhandledRejection_ = function () {
      if (this.isRejectionHandled_) return !1;
      var c = $jscomp.global.CustomEvent,
      g = $jscomp.global.Event,
      h = $jscomp.global.dispatchEvent;
      if ('undefined' === typeof h) return !0;
      'function' === typeof c ? c = new c('unhandledrejection', {
        cancelable: !0
      }) :
      'function' === typeof g ? c = new g('unhandledrejection', {
        cancelable: !0
      }) : (
        c = $jscomp.global.document.createEvent('CustomEvent'),
        c.initCustomEvent('unhandledrejection', !1, !0, c)
      );
      c.promise = this;
      c.reason = this.result_;
      return h(c)
    };
    f.prototype.executeOnSettledCallbacks_ = function () {
      if (null != this.onSettledCallbacks_) {
        for (var c = 0; c < this.onSettledCallbacks_.length; ++c) d.asyncExecute(this.onSettledCallbacks_[c]);
        this.onSettledCallbacks_ = null
      }
    };
    var d = new b;
    f.prototype.settleSameAsPromise_ = function (c) {
      var g = this.createResolveAndReject_();
      c.callWhenSettled_(g.resolve, g.reject)
    };
    f.prototype.settleSameAsThenable_ = function (c, g) {
      var h = this.createResolveAndReject_();
      try {
        c.call(g, h.resolve, h.reject)
      } catch (l) {
        h.reject(l)
      }
    };
    f.prototype.then = function (c, g) {
      function h(p, q) {
        return 'function' == typeof p ? function (u) {
          try {
            l(p(u))
          } catch (r) {
            k(r)
          }
        }
         : q
      }
      var l,
      k,
      n = new f(function (p, q) {
        l = p;
        k = q
      });
      this.callWhenSettled_(h(c, l), h(g, k));
      return n
    };
    f.prototype.catch = function (c) {
      return this.then(void 0, c)
    };
    f.prototype.callWhenSettled_ = function (c, g) {
      function h() {
        switch (l.state_) {
          case 1:
            c(l.result_);
            break;
          case 2:
            g(l.result_);
            break;
          default:
            throw Error('Unexpected state: ' + l.state_);
        }
      }
      var l = this;
      null == this.onSettledCallbacks_ ? d.asyncExecute(h) : this.onSettledCallbacks_.push(h);
      this.isRejectionHandled_ = !0
    };
    f.resolve = e;
    f.reject = function (c) {
      return new f(function (g, h) {
        h(c)
      })
    };
    f.race = function (c) {
      return new f(
        function (g, h) {
          for (var l = $jscomp.makeIterator(c), k = l.next(); !k.done; k = l.next()) e(k.value).callWhenSettled_(g, h)
        }
      )
    };
    f.all = function (c) {
      var g = $jscomp.makeIterator(c),
      h = g.next();
      return h.done ? e([]) : new f(
        function (l, k) {
          function n(u) {
            return function (r) {
              p[u] = r;
              q--;
              0 == q &&
              l(p)
            }
          }
          var p = [],
          q = 0;
          do p.push(void 0),
          q++,
          e(h.value).callWhenSettled_(n(p.length - 1), k),
          h = g.next();
          while (!h.done)
        }
      )
    };
    return f
  },
  'es6',
  'es3'
);
$jscomp.checkStringArgs = function (a, b, e) {
  if (null == a) throw new TypeError(
    'The \'this\' value for String.prototype.' + e + ' must not be null or undefined'
  );
  if (b instanceof RegExp) throw new TypeError(
    'First argument to String.prototype.' + e + ' must not be a regular expression'
  );
  return a + ''
};
$jscomp.polyfill(
  'String.prototype.startsWith',
  function (a) {
    return a ? a : function (b, e) {
      var m = $jscomp.checkStringArgs(this, b, 'startsWith');
      b += '';
      var f = m.length,
      d = b.length;
      e = Math.max(0, Math.min(e | 0, m.length));
      for (var c = 0; c < d && e < f; ) if (m[e++] != b[c++]) return !1;
      return c >= d
    }
  },
  'es6',
  'es3'
);
$jscomp.polyfill(
  'Promise.prototype.finally',
  function (a) {
    return a ? a : function (b) {
      return this.then(
        function (e) {
          return Promise.resolve(b()).then(function () {
            return e
          })
        },
        function (e) {
          return Promise.resolve(b()).then(function () {
            throw e;
          })
        }
      )
    }
  },
  'es9',
  'es3'
);
var keyLatestSubtitleLanguage = 'vjs.latest.subtitles.language',
keyVjsVolume = 'vjs.volume';
function getPlayerControlBarSettings() {
  var a = 'playToggle volumePanel CurrentTimeDisplay TimeDivider DurationDisplay CustomControlSpacer qualitySelector fullscreenToggle progressControl'.split(' '),
  b = 'volumePanel CurrentTimeDisplay TimeDivider DurationDisplay CustomControlSpacer qualitySelector fullscreenToggle progressControl'.split(' ');
  return 768 <= window.screen.width ? a : b
}
function vjsVolumeChangeHandler(a) {
  var b,
  e = null != (b = localStorage.getItem(keyVjsVolume)) ? b : '1';
  a.volume(parseFloat(e));
  a.on(
    'volumechange',
    function () {
      localStorage.setItem(keyVjsVolume, a.volume().toString())
    }
  )
}
function handlePlayerErrorMessage() {
  var a = document.querySelector('.jw-error-msg') ||
  document.querySelector('.vjs-modal-dialog-content');
  null !== a &&
  setTimeout(
    function () {
      var b = '';
      800 < window.screen.width &&
      (b += '<div class=\'only-prem-1080p\'>');
      b += '<p>If you keep seeing this message, your Internet provider is blocking us.</p><br/><p>Use a VPN (free options below):</p><p><a href=\'https://protonvpn.com/free-vpn\' target=\'_blank\'>ProtonVPN</a> OR <a href=\'https://riseup.net/en/vpn\' target=\'_blank\'>RiseUP VPN</a></p><p>Install, Connect server, and then everything will work</p> </br><p>THIS IS NOT AN AD, VPNs will bypass the restrictions</p>';
      700 <
      window.screen.width &&
      (b += '</div>');
      a.innerHTML = b
    },
    10
  )
}
function getPlayerOptions(a, b, e) {
  return {
    textTrackSettings: !0,
    persistTextTrackSettings: !0,
    playbackTitle: e,
    poster: b,
    inactivityTimeout: 4000,
    responsive: !0,
    preload: 'none',
    autoplay: !0,
    techOrder: [
      'chromecast',
      'hls',
      'html5'
    ],
    chromecast: {
      preloadTextTracks: !1,
      requestBackdropFn: function () {
        return b
      },
      requestPosterFn: function () {
        return a
      },
      requestTitleFn: function () {
        return e
      },
      requestSubtitleFn: function () {
        return 'Playing on ' + window.location.host
      }
    },
    controlBar: {
      volumePanel: {
        inline: !1
      },
      children: getPlayerControlBarSettings()
    },
    plugins: {
      landscapeFullscreen: {
        fullscreen: {
          enterOnRotate: !1,
          alwaysInLandscapeMode: !0,
          iOS: !1
        }
      },
      mobileUi: {
      },
      seekButtons: {
        forward: 768 <= window.screen.width ? 10 : 0,
        back: 768 <= window.screen.width ? 10 : 0
      },
      chromecast: {
        addButtonToControlBar: !0,
        buttonPositionIndex: window.videojs.browser.IS_IOS ||
        window.videojs.browser.IS_ANDROID ? 5 : 7
      },
      airplayButton: {
      },
      hotkeys: {
        volumeStep: 0.1,
        seekStep: 10,
        enableVolumeScroll: !1,
        enableModifiersForNumbers: !1
      }
    },
    hls: {
      preloadTextTracks: !1,
      overrideNative: !0,
      _: {
      }
    },
    html5: {
      preloadTextTracks: !1,
      overrideNative: !0
    }
  }
}
function getUrlSettings() {
  return {
    p2pDisabled: !!getAllUrlParams(window.location.href).p2pDisabled
  }
}
function MoviesPlay() {
  function a(f) {
    window.videoJS = videojs(
      'video_player',
      getPlayerOptions(
        window.movie_storage.movie_poster,
        window.movie_storage.backdrop_huge,
        window.movie_storage.title + ' (' + window.movie_storage.year + ')'
      ),
      function () {
        var d = this,
        c,
        g,
        h,
        l,
        k;
        return $jscomp.asyncExecutePromiseGeneratorProgram(
          function (n) {
            switch (n.nextAddress) {
              case 1:
                d.addClass('plyr-skin');
                d.aspectRatio('16:9');
                vjsCreateSubtitlesButton(d);
                addControlBarOverlay(d);
                playerBindQualitySave(d);
                vjsVolumeChangeHandler(d);
                vjsCreatePlaybackTitleComponent(d);
                d.on('error', handlePlayerErrorMessage);
                d.on('clickHandleTrackUpload', SubtitleUploadHandle);
                d.updatePlaybackTitle(d.options().playbackTitle);
                c = [];
                g = [];
                h = {
                  '360p': '360p',
                  '480p': '480p',
                  '720p': 'HD',
                  '1080p': 'FullHD'
                };
                for (l in h) 'undefined' !== typeof f.streams[l] &&
                (
                  null === f.streams[l] ? g.push({
                    label: h[l],
                    quality: l
                  }) : c.push({
                    src: f.streams[l],
                    label: h[l],
                    type: 'application/vnd.apple.mpegurl',
                    withCredentials: !1,
                    selected: !1
                  })
                );
                addQualityButton(d, g);
                vjsLoadDefaultSubtitleStyle(d);
                c = selectDefaultSource(c);
                ProgressLogger.createInstance(f.watchHistory || 'lookmovie.mv_' + f.id_movie);
                ProgressLogger.hasProgress() &&
                ProgressLogger.renderDialogue(
                  '#PlayerZone',
                  function (p) {
                    this.play();
                    this.currentTime(p / 1000)
                  }.bind(d),
                  function () {
                    this.play()
                  }.bind(d)
                );
                d.on(
                  'timeupdate',
                  function () {
                    ProgressLogger.log(1000 * this.currentTime(), 1000 * this.getCache().duration)
                  }.bind(d)
                );
                if (
                  'undefined' !== typeof window.p2pEngine ||
                  !window.Yii2App.p2p.enabled ||
                  e.p2pDisabled
                ) {
                  n.jumpTo(2);
                  break
                }
                n.setCatchFinallyBlocks(3);
                window.p2pEngine = new P2PEngineHls(configP2P());
                return n.yield(P2PEngineHls.tryRegisterServiceWorker(configP2P()), 5);
              case 5:
                n.leaveTryBlock(2);
                break;
              case 3:
                k = n.enterCatchBlock(),
                console.log(k);
              case 2:
                d.src(c),
                m(f.subtitles),
                document.querySelector('#PlayerZone .player__wrapper').style.display = 'block',
                document.querySelector('#PlayerZone .placeholder__wrapper').classList.add('hidden'),
                localStorage.setItem('vjs.has_played_vid', '1'),
                n.jumpToEnd()
            }
          }
        )
      }
    )
  }
  function b() {
    return new Promise(
      function (f, d) {
        (new HttpClient).get(
          '/api/v1/security/movie-access?id_movie=' + window.movie_storage.id_movie + '&hash=' + window.movie_storage.hash +
          '&expires=' + window.movie_storage.expires,
          function (c) {
            f(JSON.parse(c))
          },
          function (c) {
            d(c)
          }
        )
      }
    )
  }
  new HttpClient;
  var e = getUrlSettings(),
  m = function (f) {
    var d,
    c,
    g,
    h,
    l;
    return $jscomp.asyncExecutePromiseGeneratorProgram(
      function (k) {
        for (d = 0; d < f.length; d++) c = f[d],
        videojs.browser.IS_ANY_SAFARI &&
        'string' !== typeof c.file ||
        (
          g = 'string' === typeof c.file ? window.location.protocol + '//' + window.location.host + c.file : c.file,
          window.videoJS.addRemoteTextTrack({
            src: g,
            kind: 'subtitles',
            label: c.language
          }, !0)
        );
        h = window.localStorage.getItem(keyLatestSubtitleLanguage);
        if ('undefined' === typeof h) return k.return();
        for (l = 0; window.videoJS.textTracks().length > l; l++) if (window.videoJS.textTracks() [l].label === h) {
          window.videoJS.textTracks() [l].mode = 'showing';
          break
        }
        k.jumpToEnd()
      }
    )
  };
  $(document).ready(
    function () {
      var f,
      d;
      return $jscomp.asyncExecutePromiseGeneratorProgram(
        function (c) {
          switch (c.nextAddress) {
            case 1:
              return $('#similar-movies').owlCarousel({
                dots: !0,
                responsiveClass: !0,
                nav: !0,
                navText: [
                  '',
                  ''
                ],
                responsive: {
                  0: {
                    items: 2,
                    nav: !0,
                    slideBy: 2,
                    margin: 0
                  },
                  480: {
                    items: 2,
                    slideBy: 2,
                    margin: 0,
                    nav: !0
                  },
                  768: {
                    slideBy: 4,
                    margin: 0,
                    items: 4
                  },
                  1024: {
                    slideBy: 5,
                    margin: 0,
                    items: 5
                  }
                }
              }),
              c.setCatchFinallyBlocks(2),
              c.yield(b(), 4);
            case 4:
              f = c.yieldResult;
              if (!f.success) return c.return(renderPlayerMessage(f.message, 'error'));
              if (f.config.isPremium) {
                c.jumpTo(5);
                break
              }
              document.querySelector('.placeholder__wrapper').style.display = 'none';
              return c.yield(initPrePlaybackCounter(f.config.tortureTimeout), 6);
            case 6:
              window.dispatchEvent(new Event('initPrePlaybackCounter-event')),
              document.querySelector('.placeholder__wrapper').style.display = 'block';
            case 5:
              a(f);
              c.leaveTryBlock(0);
              break;
            case 2:
              d = c.enterCatchBlock(),
              alert(d + '. Please Refresh page and try again.'),
              c.jumpToEnd()
          }
        }
      )
    }
  )
}
function configP2P() {
  var a = {
    swFile: '/sw.p2p.js?rev=6.1',
    token: window.Yii2App.p2p.token,
    live: !1,
    strictSegmentId: !0,
    memoryCacheLimit: {
      pc: 0,
      mobile: 0
    },
    swAutoRegister: !1,
    getStats: function (b, e, m, f) {
      1 === window.Yii2App.mode &&
      (
        e = (b / (b + m) * 100).toFixed(1),
        b = (b / 1000).toFixed(2),
        m = (m / 1000).toFixed(2),
        document.querySelector('#efficiency .value').innerHTML = e + '%',
        document.querySelector('#p2p-downloaded .value').innerHTML = b + 'mb',
        document.querySelector('#http-downloaded .value').innerHTML = m + 'mb'
      )
    },
    segmentId: function (b, e, m, f) {
      b = m.split('?') [0];
      b.startsWith('http') &&
      (b = b.split('://') [1]);
      b = b.split('/');
      e = b[1];
      b.splice(0, 6);
      b = e + '/' + b.join('/');
      1 === window.Yii2App.mode &&
      (
        console.log('segmentId', b),
        f &&
        console.log('segment range:', f)
      );
      return f ? b + '|' + f : '' + b
    },
    getPeersInfo: function (b) {
      1 === window.Yii2App.mode &&
      (document.querySelector('#seeds .value').innerHTML = b.length)
    },
    channelId: function (b) {
      b = b.split('?') [0];
      b.startsWith('http') &&
      (b = b.split('://') [1]);
      b = b.split('/');
      var e = b[1];
      b.splice(0, 6);
      b = e + '/' + b.join('/');
      1 === window.Yii2App.mode &&
      console.log('channelId:', b);
      return b
    }
  };
  'string' === typeof window.Yii2App.p2p.announce &&
  (a.announce = window.Yii2App.p2p.announce);
  return a
}
function ShowsPlay() {
  function a() {
    new HttpClient;
    this.subtitles = [];
    m(
      function (h) {
        return $jscomp.asyncExecutePromiseGeneratorProgram(
          function (l) {
            switch (l.nextAddress) {
              case 1:
                if (!h.success) return renderPlayerMessage(h.message, 'error'),
                l.return();
                if (h.config.isPremium) {
                  l.jumpTo(3);
                  break
                }
                document.querySelector('.placeholder__wrapper').style.display = 'none';
                window.prePlaybackCounterPromise = initPrePlaybackCounter(h.config.tortureTimeout);
                l.setCatchFinallyBlocks(4);
                return l.yield(window.prePlaybackCounterPromise, 6);
              case 6:
                window.prePlaybackCounterPromise = void 0;
                window.dispatchEvent(new Event('initPrePlaybackCounter-event'));
                l.leaveTryBlock(3);
                break;
              case 4:
                return l.enterCatchBlock(),
                window.prePlaybackCounterPromise = void 0,
                l.return();
              case 3:
                g(h, !1),
                l.jumpToEnd()
            }
          }
        )
      }
    );
    window.addEventListener(
      'ChangedEpisodeHash',
      function () {
        location.hash.match(/#S\d+-E\d+-\d+/gi) &&
        (
          e(location.hash),
          'undefined' !== typeof window.videoJS &&
          window.videoJS.pause(),
          document.querySelector('.player__wrapper').style.display = 'none',
          document.querySelector('.placeholder__wrapper').style.display = 'block',
          c()
        )
      }
    );
    var d = function (h) {
      h.forEach(
        function (k) {
          if (videojs.browser.IS_ANY_SAFARI && 'string' !== typeof k.file) return !1;
          window.videoJS.addRemoteTextTrack({
            src: 'string' === typeof k.file ? window.location.protocol + '//' + window.location.host + k.file : k.file,
            kind: 'subtitles',
            label: k.language
          }, !0)
        }
      );
      h = window.localStorage.getItem(keyLatestSubtitleLanguage);
      if ('undefined' !== typeof h) for (var l = 0; window.videoJS.textTracks().length > l; l++) if (window.videoJS.textTracks() [l].label === h) {
        window.videoJS.textTracks() [l].mode = 'showing';
        break
      }
    },
    c = function () {
      window.UploadIndex = 1;
      if (
        'undefined' !== typeof window.videoJS &&
        'undefined' !== typeof window.videoJS.chromecastSessionManager
      ) {
        var h = window.videoJS.chromecastSessionManager.getCastContext();
        'CONNECTED' === h.getCastState() &&
        h.endCurrentSession(!0)
      }
      m(
        function (l) {
          return $jscomp.asyncExecutePromiseGeneratorProgram(
            function (k) {
              switch (k.nextAddress) {
                case 1:
                  if (!l.success) {
                    renderPlayerMessage(l.message, 'error');
                    k.jumpTo(0);
                    break
                  }
                  window.subtitles = l.subtitles;
                  if (l.config.isPremium) {
                    k.jumpTo(3);
                    break
                  }
                  'undefined' !== typeof window.prePlaybackCounterPromise &&
                  window.prePlaybackCounterPromise.cancel();
                  document.querySelector('.placeholder__wrapper').style.display = 'none';
                  window.prePlaybackCounterPromise = initPrePlaybackCounter(l.config.tortureTimeout);
                  k.setCatchFinallyBlocks(4);
                  return k.yield(window.prePlaybackCounterPromise, 6);
                case 6:
                  window.prePlaybackCounterPromise = void 0;
                  window.dispatchEvent(new Event('initPrePlaybackCounter-event'));
                  k.leaveTryBlock(3);
                  break;
                case 4:
                  return k.enterCatchBlock(),
                  window.prePlaybackCounterPromise = void 0,
                  k.return();
                case 3:
                  'undefined' !== typeof window.videoJS &&
                  window.videoJS.dispose(),
                  g(l, !0),
                  k.jumpToEnd()
              }
            }
          )
        }
      )
    },
    g = function (h, l) {
      var k = document.querySelector('.continue-wrapper');
      'undefined' !== typeof k &&
      null !== k &&
      (k.style.display = 'none');
      k = document.createElement('video');
      k.classList.add('video-js');
      k.classList.add('vjs-lookmovie');
      k.classList.add('vjs-default-skin');
      k.classList.add('vjs-fluid');
      k.setAttribute('webkit-playsinline', 'true');
      k.setAttribute('controls', '');
      k.setAttribute('playsinline', 'true');
      k.setAttribute('x-webkit-airplay', 'allow');
      k.setAttribute('id', 'video_player');
      document.querySelector('#player-container').appendChild(k);
      window.videoJS = videojs(
        'video_player',
        getPlayerOptions(
          window.show_storage.poster_medium,
          window.show_storage.backdrop_huge,
          window.show_storage.title + ' (' + window.show_storage.year + ')'
        ),
        function () {
          var n = this,
          p,
          q,
          u,
          r,
          w;
          return $jscomp.asyncExecutePromiseGeneratorProgram(
            function (t) {
              switch (t.nextAddress) {
                case 1:
                  n.addClass('plyr-skin');
                  n.aspectRatio('16:9');
                  vjsCreateSubtitlesButton(n);
                  playerBindQualitySave(n);
                  vjsVolumeChangeHandler(n);
                  addControlBarOverlay(n);
                  vjsCreatePlaybackTitleComponent(n);
                  vjsCreateNextEpisodeButton(n);
                  (
                    videojs.browser.IS_ANDROID ||
                    videojs.browser.IS_IOS ||
                    videojs.browser.IS_IPAD
                  ) &&
                  n.controlBar.volumePanel.hide();
                  vjsLoadDefaultSubtitleStyle(n);
                  window.currentEpisodeIndex + 1 < window.show_storage.total_episodes &&
                  n.upnext({
                    timeout: 6000,
                    headText: 'NEXT EPISODE',
                    cancelText: 'Cancel',
                    getTitle: function () {
                      var v = window.show_storage.seasons[window.currentEpisodeIndex +
                      1];
                      return 'S' + v.season + 'E' + v.episode + ' ' + v.title
                    },
                    next: function () {
                      window.dispatchEvent(new Event('switch-next-episode'))
                    }
                  });
                  n.updatePlaybackTitle(
                    'S' + show_storage.seasons[window.currentEpisodeIndex].season + 'E' + show_storage.seasons[window.currentEpisodeIndex].episode + ' ' + show_storage.seasons[window.currentEpisodeIndex].title
                  );
                  n.on('clickHandleTrackUpload', SubtitleUploadHandle);
                  n.on('error', handlePlayerErrorMessage);
                  l &&
                  window.videoJS.play();
                  p = [];
                  q = [];
                  u = {
                    360: '360p',
                    480: '480p',
                    720: 'HD',
                    1080: 'FullHD'
                  };
                  for (r in u) 'undefined' !== typeof h.streams[r] &&
                  (
                    null === h.streams[r] ? q.push({
                      label: u[r],
                      quality: r
                    }) : p.push({
                      src: h.streams[r],
                      label: u[r],
                      type: 'application/vnd.apple.mpegurl',
                      withCredentials: !1,
                      selected: !1
                    })
                  );
                  addQualityButton(n, q);
                  p = selectDefaultSource(p);
                  ProgressLogger.createInstance(h.watchHistory || 'ep_' + h.id_episode);
                  ProgressLogger.hasProgress() &&
                  (
                    n.pause(),
                    ProgressLogger.renderDialogue(
                      '#PlayerZone',
                      function (v) {
                        this.play();
                        this.currentTime(v / 1000)
                      }.bind(n),
                      function () {
                        this.play()
                      }.bind(n)
                    )
                  );
                  n.on(
                    'timeupdate',
                    function () {
                      ProgressLogger.log(1000 * this.currentTime(), 1000 * this.getCache().duration)
                    }.bind(n)
                  );
                  window.videoJS.on('subsUploader', function () {
                    SubtitleUploadHandle()
                  });
                  if (
                    'undefined' !== typeof window.p2pEngine ||
                    !window.Yii2App.p2p.enabled ||
                    b.p2pDisabled
                  ) {
                    t.jumpTo(2);
                    break
                  }
                  t.setCatchFinallyBlocks(3);
                  window.p2pEngine = new P2PEngineHls(configP2P());
                  return t.yield(P2PEngineHls.tryRegisterServiceWorker(configP2P()), 5);
                case 5:
                  t.leaveTryBlock(2);
                  break;
                case 3:
                  w = t.enterCatchBlock(),
                  console.log(w);
                case 2:
                  n.src(p),
                  d(h.subtitles),
                  document.querySelector('.player__wrapper').style.display = 'block',
                  document.querySelector('.placeholder__wrapper').style.display = 'none',
                  localStorage.setItem('vjs.has_played_vid', '1'),
                  t.jumpToEnd()
              }
            }
          )
        }
      )
    }
  }
  var b = getUrlSettings(),
  e = function (d) {
    d = d.substr(1).split('-');
    'undefined' !== typeof d['0'] &&
    'undefined' !== typeof d['1'] &&
    (
      window.currentSeason = d['0'].substr(1),
      window.currentEpisode = d['1'].substr(1),
      window.currentEpisodeID = d['2']
    )
  },
  m = function (d) {
    var c = new HttpClient,
    g = '/api/v1/security/episode-access?id_episode=' +
    window.currentEpisodeID + '&hash=' + window.show_storage.hash + '&expires=' + window.show_storage.expires;
    'undefined' !== typeof window.prePlaybackCounterPromise &&
    (
      window.prePlaybackCounterPromise.cancel(),
      window.prePlaybackCounterPromise = void 0
    );
    c.get(
      g,
      function (h) {
        h = JSON.parse(h);
        d(h);
        var l = {},
        k;
        for (k in h.streams) h.hasOwnProperty(k) &&
        (l[k + 'p'] = h[k]);
        window.QualityLevels = l
      }
    )
  },
  f = function () {
    window.VueEpisodesSwitcher = new Vue({
      el: '#episodes-switcher',
      name: 'EpisodesSwitcher',
      data: {
        currentSeason: 0,
        currentEpisode: 0,
        seasons: [],
        isOpenSeasons: !1,
        isOpenEpisodes: !1
      },
      computed: {
        seasonEpisodes: function () {
          return this.seasons[this.currentSeason].episodes
        }
      },
      methods: {
        isValidHash: function (d) {
          return d.match(/#S\d+-E\d+-\d+/gi) ? !0 : !1
        },
        setFromHash: function (d) {
          d = d.substr(1).split('-');
          'undefined' !== typeof d['0'] &&
          'undefined' !== typeof d['1'] &&
          (
            this.currentSeason = d['0'].substr(1),
            this.currentEpisode = d['1'].substr(1),
            window.currentEpisodeIndex = this.seasons[this.currentSeason].episodes[this.currentEpisode].index
          )
        },
        updateHash: function () {
          if (window.history && window.history.pushState) {
            var d = location.href;
            d = 'undefined' !== typeof location.hash &&
            location.hash &&
            this.isValidHash(location.hash) ? d.replace(
              location.hash,
              '#S' + this.currentSeason + '-E' + this.currentEpisode + '-' + this.seasons[this.currentSeason].episodes[this.currentEpisode].id_episode
            ) : d + '#S' + this.currentSeason + '-E' + this.currentEpisode + '-' + this.seasons[this.currentSeason].episodes[this.currentEpisode].id_episode;
            history.replaceState(null, null, d);
            d = document.createEvent('Event');
            d.initEvent('ChangedEpisodeHash', !0, !0);
            window.dispatchEvent(d)
          } else location.replace(
            '#S' + this.currentSeason + '-E' + this.currentEpisode + '-' + this.seasons[this.currentSeason].episodes[this.currentEpisode].id_episode
          ),
          d = document.createEvent('Event'),
          d.initEvent('ChangedEpisodeHash', !0, !0),
          window.dispatchEvent(d);
          window.currentEpisodeID = this.seasons[this.currentSeason].episodes[this.currentEpisode].id_episode;
          window.currentEpisodeIndex = this.seasons[this.currentSeason].episodes[this.currentEpisode].index
        },
        updateComponent: function () {
          'undefined' !==
          typeof location.hash &&
          '' !== location.hash &&
          this.isValidHash(location.hash) ? this.setFromHash(location.hash) : this.setRecentPlayedEpisode() ||
          this.setLatest()
        },
        updateSeason: function (d) {
          this.currentSeason = d;
          this.setLatestEpisode(d);
          this.updateHash()
        },
        updateEpisode: function (d) {
          this.currentEpisode = d;
          this.updateHash()
        },
        setRecentPlayedEpisode: function () {
          if ('undefined' !== typeof window.show_storage.recentEpisode) {
            for (var d = 0; d < window.show_storage.seasons.length; d++) if (
              window.show_storage.seasons[d].id_episode ===
              window.show_storage.recentEpisode.id_episode
            ) return this.currentEpisode = window.show_storage.seasons[d].episode,
            this.currentSeason = window.show_storage.seasons[d].season,
            window.currentEpisodeIndex = window.show_storage.seasons[d].index,
            this.updateHash(),
            !0;
            return !1
          }
        },
        setLatestEpisode: function () {
          var d = 1,
          c;
          for (c in this.seasons[this.currentSeason].episodes) {
            d = c;
            break
          }
          this.currentEpisode = d
        },
        setLatest: function (d) {
          d = 'undefined' === typeof d ? 1 : d;
          var c = 1;
          if (1 === d) for (var g in this.seasons) {
            d = g;
            break
          }
          for (var h in this.seasons[d].episodes) {
            c = h;
            break
          }
          window.currentEpisodeIndex = this.seasons[d].episodes[c].index;
          this.currentEpisode = c;
          this.currentSeason = d;
          this.updateHash()
        }
      },
      created: function () {
        this.seasons = JSON.parse(window.seasons)
      },
      mounted: function () {
        var d = this;
        window.addEventListener(
          'switch-next-episode',
          function () {
            d.currentSeason = window.show_storage.seasons[window.currentEpisodeIndex + 1].season;
            d.currentEpisode = window.show_storage.seasons[window.currentEpisodeIndex + 1].episode;
            d.updateHash()
          }
        );
        document.querySelector('.episodes-controls-wrapper').classList.remove('zero-opacity');
        this.updateComponent()
      }
    })
  };
  location.hash.match(/#S\d+-E\d+-\d+/gi) &&
  e(location.hash);
  window.addEventListener('ChangedEpisodeHash', function () {
    e(location.hash)
  });
  $(document).ready(
    function () {
      return $jscomp.asyncExecutePromiseGeneratorProgram(
        function (d) {
          try {
            f(),
            a()
          } catch (c) {
            console.log(c)
          }
          d.jumpToEnd()
        }
      )
    }
  )
}
function SubtitleUploadHandle() {
  window.UploadIndex = 'undefined' === typeof window.UploadIndex ? 1 : window.UploadIndex;
  var a = function () {
    for (var b = 0; b < window.videoJS.tech_.textTracks_.length; b++) window.videoJS.tech_.textTracks_[b].mode = 'disabled'
  };
  (
    function () {
      for (
        var b = document.querySelector('#SubtitlesDropzone');
        b.firstChild;
      ) b.removeChild(b.firstChild);
      var e = document.createElement('input');
      e.type = 'file';
      b.appendChild(e);
      e.onchange = function () {
        var m = new XMLHttpRequest,
        f = new FormData;
        f.append('srt', e.files[0]);
        m.open(
          'POST',
          window.location.protocol + '//' + window.Yii2App.apiDomain + '/v1/subtitles/srt-to-vtt'
        );
        m.send(f);
        m.onreadystatechange = function () {
          if (4 === m.readyState) {
            var d = new Blob([m.response], {
              type: 'text/vtt'
            });
            d = URL.createObjectURL(d);
            var c = window.videoJS.addRemoteTextTrack({
              src: d,
              label: 'Custom'
            }, !1);
            a();
            setTimeout(function () {
              c.track.mode = 'showing'
            }, 50);
            window.UploadIndex++
          }
        }
      };
      e.click()
    }
  ) ()
}
function addControlBarOverlay(a) {
  var b = new (videojs.getComponent('Component')) (a);
  b.addClass('vjs-control-bar-overlay');
  a.addChild(b)
}
function playerBindQualitySave(a) {
  a.on(
    'qualitySelected',
    function (b, e) {
      window.localStorage.setItem('lookmovie.prevQuality', e.label)
    }
  )
}
function episodeNextCard(a, b) {
  b = b ||
  {
  };
  b.title = 'undefined' !== typeof b.title ? '' : b.title;
  b.timeout = 'undefined' !== typeof b.timeout ? 10 : b.timeout;
  b.threshold = 'undefined' !== typeof b.threshold ? 10 : b.threshold;
  b.backdrop = 'undefined' !== typeof b.backdrop ? '' : b.backdrop;
  b = new (videojs.getComponent('Component')) (
    a,
    {
      el: videojs.dom.createEl('div', {
        type: 'vjs-next-card'
      }, {
        class : 'vjs-next-card'
      })
    }
  );
  a.addChild(b)
}
function vjsCreatePlaybackTitleComponent(a) {
  var b = new (videojs.getComponent('Component')) (
    a,
    {
      el: videojs.dom.createEl('div', {
        type: 'vjs-title'
      }, {
        class : 'vjs-title'
      })
    }
  );
  a.addChild(b);
  a.updatePlaybackTitle = function (e) {
    'undefined' !== typeof b &&
    null !== b &&
    (b.contentEl().innerHTML = e)
  }
}
function addQualityButton(a, b) {
  a.on(
    'playing',
    function () {
      null !== document.querySelector('.only-prem-1080p') &&
      $('.only-prem-1080p').remove()
    }
  );
  $(document).on(
    'click',
    '.only-prem-1080p__continue-watching',
    function (e) {
      e.preventDefault();
      a.play()
    }
  );
  a.on(
    'playerSourcesChanged',
    function () {
      for (var e = 0; e < b.length; e++) {
        var m = $(
          '<li class="vjs-menu-item" id="user-limited-notify-button">' + b[e].label + '</li>'
        );
        m.on(
          'click',
          function () {
            null === document.querySelector('.only-prem-1080p') &&
            (
              700 < window.screen.width ? $('.vjs-lookmovie').append(get1080pMessage()) :
              $('.vjs-lookmovie').append(get1080pMessageMobile())
            );
            a.pause();
            videoJS.controlBar.qualitySelector.handleClick()
          }
        );
        $('.vjs-settings-quality-menu .vjs-menu-content').append(m)
      }
    }
  )
}
function get1080pMessage() {
  return '<div class="only-prem-1080p"><p><b>720p</b> and <b>FullHD</b> is available for Premium Members only</p><br><p><br>While we are doing everything to provide the best FREE <br>streaming experience it still comes with high server bandwidth costs.</p><br><p>To reduce costs, some features will be available for Paid Members only.</p><br><div class="only-prem-1080p__buttons"><a class="only-prem-1080p__button only-prem-1080p__continue-watching" href="#vjs-continue-watching">Continue Watching</a><a class="only-prem-1080p__button only-prem-1080p__check-premium" href="' + Yii2App.PREMIUM_PAGE_URL +
  '" target="_blank">Become Premium</a></div></div>'
}
function get1080pMessageMobile() {
  return '<div class="only-prem-1080p"><p><b>720p</b> and <b>FullHD</b> is available for Premium Members</p><br><div class="only-prem-1080p__buttons"><a class="only-prem-1080p__button only-prem-1080p__continue-watching" href="#vjs-continue-watching">Continue Watching</a><a class="only-prem-1080p__button only-prem-1080p__check-premium" href="' + Yii2App.PREMIUM_PAGE_URL + '" target="_blank">Become Premium</a></div></div>'
}
function setVideoJsBreakpoints(a) {
  a.breakpoints({
    tiny: 30,
    xsmall: 320,
    small: 450,
    medium: 992,
    large: 1440,
    xlarge: Infinity
  })
}
function vjsLoadDefaultSubtitleStyle(a) {
  return !1
}
function selectDefaultSource(a) {
  var b = [
    'FullHD',
    'HD',
    '480p',
    '360p'
  ],
  e = a.map(function (d) {
    return d.label
  }),
  m = window.localStorage.getItem('lookmovie.prevQuality');
  if (1 === a.length) return a['0'].selected = !0,
  a;
  if ('string' === typeof m) {
    var f = e.indexOf(m);
    if ( - 1 !== f) return a[f].selected = !0,
    a;
    for (f = 0; f < b.length; f++) if (b.indexOf(b[f]) >= b.indexOf(m) && - 1 !== e.indexOf(b[f])) return a[e.indexOf(b[f])].selected = !0,
    a
  }
  for (b = 0; b < a.length; b++) if ('480p' === a[b].label) return a[b].selected = !0,
  a;
  a['0'].selected = !0;
  return a
}
function vjsCreateNextEpisodeButton(a) {
  a = a.controlBar.addChild(
    'button',
    {
      text: 'Next Episode',
      controlText: 'Next Episode',
      className: 'vjs-next-episode-button'
    },
    1
  );
  a.haveNextEpisode = function () {
    return window.currentEpisodeIndex + 1 < window.show_storage.total_episodes ? !0 : !1
  };
  a.haveNextEpisode() ||
  a.disable();
  a.handleClick = function () {
    window.dispatchEvent(new Event('switch-next-episode'))
  }
}
function vjsCreateSubtitlesButton(a) {
  var b = videojs.getComponent('Component'),
  e = videojs.getComponent('Button');
  this.enableTextTrack = function (c, g) {
    var h = this,
    l,
    k,
    n,
    p;
    return $jscomp.asyncExecutePromiseGeneratorProgram(
      function (q) {
        switch (q.nextAddress) {
          case 1:
            for (l = 0; a.textTracks().length > l; l++) a.textTracks() [l].mode = 'disabled';
            if ('object' != typeof a.textTracks() [c].src) {
              q.jumpTo(2);
              break
            }
            q.setCatchFinallyBlocks(3);
            a.notifyEl = vjsInsertInTextTrackDisplay(a, 'Loading: ' + g);
            k = new FormData;
            k.append('source_id', a.textTracks() [c].src[1]);
            k.append('opensubtitle_id', a.textTracks() [c].src[0]);
            k.append('language_id', a.textTracks() [c].src[2]);
            k.append('release_title', a.textTracks() [c].src[3]);
            k.append('score', a.textTracks() [c].src[4]);
            return q.yield(h.uploadTextTrack(k), 5);
          case 5:
            n = q.yieldResult;
            n = 'string' === typeof n ? JSON.parse(n) : n;
            a.textTracks() [c].src = n.url;
            q.leaveTryBlock(2);
            break;
          case 3:
            return p = q.enterCatchBlock(),
            console.log(p),
            'undefined' !== typeof a.notifyEl &&
            !1 === a.notifyEl.isDisposed() &&
            a.notifyEl.dispose(),
            q.return(!1);
          case 2:
            if ('undefined' === typeof a.textTracks() [c]) return q.return(!1);
            a.textTracks() [c].mode = 'showing';
            window.localStorage.setItem(keyLatestSubtitleLanguage, a.textTracks() [c].label);
            'undefined' !== typeof a.notifyEl &&
            !1 === a.notifyEl.isDisposed() &&
            a.notifyEl.dispose();
            q.jumpToEnd()
        }
      }
    )
  };
  this.uploadTextTrack = function (c) {
    var g;
    'undefined' !== typeof window.movie_storage &&
    'movie' === window.movie_storage.type ? g = '/api/v1/security/upload-movie-opensub?id_movie=' + window.movie_storage.id_movie : 'undefined' !==
    typeof window.show_storage &&
    'show' === window.show_storage.type &&
    (
      g = '/api/v1/security/upload-episode-opensub?id_episode=' + window.currentEpisodeID
    );
    return makeAJAXCall('POST', g, null, c)
  };
  this.disableSubtitles = function (c) {
    c = a.remoteTextTracks();
    for (var g = 0; c.length > g; g++) c[g].mode = 'disabled';
    window.localStorage.removeItem(keyLatestSubtitleLanguage)
  };
  this._createSubtitleOffComponent = function () {
    var c = this,
    g = new b(a);
    g.addClass('vjs-subtitle-off');
    g.addClass('showing');
    g.addChild(
      new e(
        a,
        {
          el: function (h, l) {
            var k = document.createElement('button');
            k.innerHTML = h;
            k.classList.add.apply(k.classList, $jscomp.arrayFromIterable(l));
            return k
          }('Off', []),
          clickHandler: function (h) {
            c.disableSubtitles();
            g.trigger('closeMenuClicked');
            g.addClass('showing')
          }
        }
      )
    );
    return g
  };
  this._createHeadingComponent = function () {
    var c = function (h, l) {
      var k = document.createElement('button');
      k.innerHTML = h;
      k.classList.add('vjs-heading-button');
      k.classList.add.apply(k.classList, $jscomp.arrayFromIterable(l));
      return k
    },
    g = new b(a);
    g.addClass('vjs-settings-heading');
    g.addChild(
      new e(
        a,
        {
          el: c('Upload', []),
          clickHandler: function (h) {
            g.trigger('closeMenuClicked');
            SubtitleUploadHandle()
          }
        }
      )
    );
    g.addChild(
      new e(
        a,
        {
          el: c('Settings', []),
          clickHandler: function (h) {
            g.trigger('closeMenuClicked');
            a.getChild('textTrackSettings').open()
          }
        }
      )
    );
    g.addChild(
      new e(
        a,
        {
          el: c(
            '<span class="vjs-icon-placeholder vjs-icon-close"></span>',
            [
              'heading-close-button'
            ]
          ),
          clickHandler: function (h) {
            g.trigger('closeMenuClicked')
          }
        }
      )
    );
    return g
  };
  this._createSubtitlesListComponent = function (c) {
    var g = document.createElement('div');
    g.classList.add('vjs-subtitles-list');
    var h = new b(a, {
      el: g
    }),
    l = a.textTracks();
    g.toggleChildren = function (n) {
      for (
        var p = n.classList.contains('open'),
        q = 0;
        q < g.children.length;
        q++
      ) g.children[q].classList.remove('open');
      p ? n.classList.remove('open') : n.classList.add('open')
    };
    var k = function () {
      g.innerHTML = '';
      a.remoteTextTrackEls();
      for (var n = a.textTracks(), p = {}, q = 0; q < n.length; q++) 'subtitles' === n[q].kind &&
      (
        'undefined' === typeof p[n[q].label] &&
        (p[n[q].label] = []),
        p[n[q].label].push({
          label: n[q].label,
          mode: n[q].mode,
          index: q
        })
      );
      n = {};
      for (var u in p) {
        n.$jscomp$loop$prop$key$15 = u;
        if (p.hasOwnProperty(n.$jscomp$loop$prop$key$15)) {
          var r = document.createElement('button');
          n.$jscomp$loop$prop$menuContainer$14 = document.createElement('div');
          q = document.createElement('ul');
          r.classList.add('vjs-subtitles-language-toggle');
          r.innerHTML = '<span class="vjs-subtitles-language-title">' + n.$jscomp$loop$prop$key$15 + '</span> <span class="toggle-plus">+</span> <span class="toggle-minus">-</span>';
          n.$jscomp$loop$prop$menuContainer$14.classList.add('vjs-subtitles-item');
          n.$jscomp$loop$prop$menuContainer$14.appendChild(r);
          n.$jscomp$loop$prop$menuContainer$14.appendChild(q);
          n.$jscomp$loop$prop$menuContainer$14.addEventListener(
            'click',
            function (t) {
              return function (v) {
                v.stopPropagation();
                g.toggleChildren(t.$jscomp$loop$prop$menuContainer$14)
              }
            }(n)
          );
          q.classList.add('vjs-subtitles-language-items');
          for (
            r = {
              $jscomp$loop$prop$i$3$17: 0
            };
            r.$jscomp$loop$prop$i$3$17 < p[n.$jscomp$loop$prop$key$15].length;
            r = {
              $jscomp$loop$prop$i$3$17: r.$jscomp$loop$prop$i$3$17
            },
            r.$jscomp$loop$prop$i$3$17++
          ) {
            var w = document.createElement('li');
            w.classList.add('vjs-subtitles-language-item');
            w.innerHTML = '<span>' + n.$jscomp$loop$prop$key$15 + ' ' + (r.$jscomp$loop$prop$i$3$17 + 1) + '</span>';
            w.addEventListener(
              'click',
              function (t, v) {
                return function (x) {
                  $(x.target).parent().addClass('active-child');
                  x.stopPropagation();
                  this.enableTextTrack(
                    p[t.$jscomp$loop$prop$key$15][v.$jscomp$loop$prop$i$3$17].index,
                    t.$jscomp$loop$prop$key$15 + ' ' + (v.$jscomp$loop$prop$i$3$17 + 1) + '<svg class="nf-spinner" viewBox="0 0 50 50"><circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="5"></circle></svg>'
                  );
                  c.hide()
                }
              }(n, r).bind(this)
            );
            'showing' === p[n.$jscomp$loop$prop$key$15][r.$jscomp$loop$prop$i$3$17].mode &&
            (
              n.$jscomp$loop$prop$menuContainer$14.classList.add('open'),
              w.classList.add('showing'),
              document.getElementsByClassName('vjs-subtitle-off ') [0].classList.remove('showing')
            );
            q.appendChild(w)
          }
          g.appendChild(n.$jscomp$loop$prop$menuContainer$14)
        }
        n = {
          $jscomp$loop$prop$menuContainer$14: n.$jscomp$loop$prop$menuContainer$14,
          $jscomp$loop$prop$key$15: n.$jscomp$loop$prop$key$15
        }
      }
    };
    l.on('change', function () {
      k()
    });
    l.on('addtrack', function () {
      k()
    });
    return h
  };
  this._buildVjsMenuSwitcher = function () {
    var c = this._createHeadingComponent(),
    g = new b(a);
    g.el_.className = 'vjs-settings-body';
    var h = this._createSubtitleOffComponent();
    c.on('closeMenuClicked', function () {
      l.hide()
    });
    h.on('closeMenuClicked', function () {
      l.hide()
    });
    var l = new b(a);
    l.el_.className = 'vjs-menu vjs-settings-menu vjs-hidden';
    l.show = function () {
      this.removeClass('vjs-hidden')
    };
    l.toggle = function (k) {
      this.hasClass('vjs-hidden') ? this.show() : this.hide()
    };
    l.hide = function () {
      this.addClass('vjs-hidden')
    };
    l.addChild(c);
    g.addChild(h);
    g.addChild(this._createSubtitlesListComponent(l));
    l.addChild(g);
    return l
  };
  var m = new b(a),
  f = this._buildVjsMenuSwitcher(),
  d = new e(a, {
    clickHandler: function (c) {
      f.toggle(d)
    }
  });
  d.el_.className = 'vjs-subtitles-button vjs-menu-button vjs-menu-button-popup vjs-control vjs-button';
  m.addClass('vjs-menu-button');
  m.addClass('vjs-button');
  m.addClass('vjs-control');
  m.addChild(d);
  a.controlBar.addChild(m, {
  }, a.controlBar.children().length - 3);
  540 > window.innerWidth ? a.addChild(f) : a.controlBar.addChild(f)
}
function initPrePlaybackCounter(a) {
  a = void 0 === a ? 10 : a;
  var b;
  console.log(a);
  for (
    var e = document.querySelector('.player-pre-init-ads'),
    m = document.querySelectorAll('.player-pre-init-ads_timer__value'),
    f = document.querySelectorAll('.pre-init-ads--back-button'),
    d = document.querySelector('.pre-init-ads--loading-please-wait'),
    c = document.querySelectorAll('.pre-init-ads--close'),
    g = document.querySelectorAll('.player-pre-init-ads_timer'),
    h = 0;
    h < c.length;
    h++
  ) c[h].classList.contains('tw-hidden') ||
  c[h].classList.add('tw-hidden');
  for (h = 0; h < f.length; h++) f[h].classList.contains('tw-hidden') ||
  f[h].classList.add('tw-hidden');
  'undefined' !== typeof window._progressInterval &&
  clearInterval(window._progressInterval);
  h = new Promise(
    function (l, k) {
      b = k;
      var n = a;
      if (
        1 > n ||
        'number' === typeof window._preInitAdsTimestamp &&
        50000 > Date.now() - window._preInitAdsTimestamp
      ) return l();
      disableWindowScroll();
      e.classList.remove('tw-hidden');
      for (k = 0; k < g.length; k++) g[k].classList.remove('tw-hidden');
      null != d &&
      d.classList.remove('tw-hidden');
      'undefined' !== typeof window._counterTimeout &&
      (
        clearInterval(window._counterTimeout),
        window._counterTimeout = void 0
      );
      for (k = 0; k < c.length; k++) c[k].onclick = function () {
        window._preInitAdsTimestamp = Date.now();
        e.classList.add('tw-hidden');
        return l()
      };
      for (k = 0; k < f.length; k++) f[k].onclick = function () {
        window._preInitAdsTimestamp = Date.now();
        e.classList.add('tw-hidden');
        return l()
      };
      for (k = 0; k < m.length; k++) m[k].innerHTML = n;
      window._counterTimeout = setInterval(
        function () {
          --n;
          console.log(n);
          for (var p = 0; p < m.length; p++) m[p].innerHTML = n;
          if (0 >= n) {
            clearInterval(window._counterTimeout);
            e.classList.add('finished');
            for (p = 0; p < c.length; p++) c[p].classList.remove('tw-hidden');
            for (p = 0; p < f.length; p++) f[p].classList.remove('tw-hidden');
            null !== d &&
            d.classList.add('tw-hidden');
            for (p = 0; p < g.length; p++) g[p].classList.add('tw-hidden')
          }
        },
        1000
      )
    }
  );
  h.finally(function () {
    enableWindowScroll()
  });
  h.cancel = function () {
    clearInterval(window._counterTimeout);
    window._counterTimeout = void 0;
    b()
  };
  return h
}
function renderPlayerMessage(a, b) {
  document.querySelector('.placeholder__wrapper').style.display = 'none';
  document.querySelector('.player__wrapper').style.display = 'none';
  document.querySelector('.player_message_wrapper').style.display = 'block';
  document.querySelector('.player_message_wrapper .message-container').innerHTML = a
};
/*** END FILE: /var/www/lookmovie.ag/v3.9.36/web/assets/330939e7/js/main/player/Player.js.tmp ***/
