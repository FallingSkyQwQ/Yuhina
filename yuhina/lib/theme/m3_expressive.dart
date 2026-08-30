// Material 3 Expressive helpers: large radii, tonal surfaces, motion curves.
//
// Yuhina uses the M3 Expressive direction shipped in Flutter 3.35+:
// filled/outlined `Card`, pill NavigationBar indicator, 28px dialog radius,
// tonal (surfaceContainer) cards and expressive motion.

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

/// Corner radius used for cards, sheets and dialogs (Expressive large radii).
const double kRadiusCard = 28;
const double kRadiusDialog = 28;
const double kRadiusControl = 20;

/// Motion curve for page / element transitions (fast-out-slow-in based).
const Curve kMotionCurve = Curves.easeOutCubic;
const Duration kMotionFast = Duration(milliseconds: 160);
const Duration kMotionMedium = Duration(milliseconds: 300);

/// Surface tint used for tonal cards.
const double kTonalElevation = 1.0;

/// Builds a tonal filled card (the M3 Expressive `FilledCard` equivalent).
Widget tonalCard({
  required BuildContext context,
  Widget? child,
  EdgeInsetsGeometry? padding,
  VoidCallback? onTap,
}) {
  final card = Card.filled(
    margin: EdgeInsets.zero,
    clipBehavior: Clip.antiAlias,
    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(kRadiusCard)),
    child: padding == null ? child : Padding(padding: padding, child: child),
  );
  if (onTap == null) return card;
  return InkWell(
    onTap: onTap,
    borderRadius: BorderRadius.circular(kRadiusCard),
    child: card,
  );
}

/// Applies an expressive page-transition. Returns a `CustomTransitionPage`
/// (a `Page`) so it can be used directly by go_router's `pageBuilder`.
Page<T> expressivePageRoute<T>(Widget page) {
  return CustomTransitionPage<T>(
    transitionDuration: kMotionMedium,
    reverseTransitionDuration: kMotionFast,
    transitionsBuilder: (_, animation, _, child) {
      final curved = CurvedAnimation(parent: animation, curve: kMotionCurve);
      return FadeTransition(
        opacity: curved,
        child: SlideTransition(
          position: Tween<Offset>(
            begin: const Offset(0.02, 0.02),
            end: Offset.zero,
          ).animate(curved),
          child: child,
        ),
      );
    },
    child: page,
  );
}