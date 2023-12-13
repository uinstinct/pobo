// This file was auto-generated by 'typesafe-i18n'. Any manual changes will be overwritten.
/* eslint-disable */
import type { BaseTranslation as BaseTranslationType, LocalizedString, RequiredParams } from 'typesafe-i18n'

export type BaseTranslation = BaseTranslationType
export type BaseLocale = 'en'

export type Locales =
	| 'en'

export type Translation = RootTranslation

export type Translations = RootTranslation

type RootTranslation = {
	/**
	 * H​i​ ​{​n​a​m​e​}​!​ ​P​l​e​a​s​e​ ​l​e​a​v​e​ ​a​ ​s​t​a​r​ ​i​f​ ​y​o​u​ ​l​i​k​e​ ​t​h​i​s​ ​p​r​o​j​e​c​t​:​ ​h​t​t​p​s​:​/​/​g​i​t​h​u​b​.​c​o​m​/​i​v​a​n​h​o​f​e​r​/​t​y​p​e​s​a​f​e​-​i​1​8​n
	 * @param {string} name
	 */
	HI: RequiredParams<'name'>
	/**
	 * S​t​a​r​t​ ​S​e​s​s​i​o​n
	 */
	START_SESSION: string
	/**
	 * S​t​o​p​ ​S​e​s​s​i​o​n
	 */
	STOP_SESSION: string
	/**
	 * N​e​x​t​ ​S​e​s​s​i​o​n
	 */
	NEXT_SESSION: string
	/**
	 * Q​u​i​c​k​ ​S​e​s​s​i​o​n
	 */
	QUICK_SESSION: string
	/**
	 * S​t​o​p​ ​C​o​o​l​d​o​w​n
	 */
	STOP_COOLDOWN: string
}

export type TranslationFunctions = {
	/**
	 * Hi {name}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n
	 */
	HI: (arg: { name: string }) => LocalizedString
	/**
	 * Start Session
	 */
	START_SESSION: () => LocalizedString
	/**
	 * Stop Session
	 */
	STOP_SESSION: () => LocalizedString
	/**
	 * Next Session
	 */
	NEXT_SESSION: () => LocalizedString
	/**
	 * Quick Session
	 */
	QUICK_SESSION: () => LocalizedString
	/**
	 * Stop Cooldown
	 */
	STOP_COOLDOWN: () => LocalizedString
}

export type Formatters = {}
