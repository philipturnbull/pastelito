/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
/* eslint-disable @typescript-eslint/ban-types */
import * as $wcm from '@vscode/wasm-component-model';
import type { u32, i32, ptr } from '@vscode/wasm-component-model';

export namespace Types {
	export type Range = {
		startLine: u32;
		startChar: u32;
		endLine: u32;
		endChar: u32;
	};

	export type Warning = {
		message: string;
		range: Range;
	};

	export type Measurement = {
		key: u32;
		range: Range;
	};

	export type Results = {
		warnings: Warning[];
		measurements: Measurement[];
	};
}
export type Types = {
};
export namespace pastelito {
	export type Results = Types.Results;
	export type Imports = {
	};
	export namespace Imports {
		export type Promisified = $wcm.$imports.Promisify<Imports>;
	}
	export namespace imports {
		export type Promisify<T> = $wcm.$imports.Promisify<T>;
	}
	export type Exports = {
		applyDefaultRules: (input: string) => Results;
	};
	export namespace Exports {
		export type Promisified = $wcm.$exports.Promisify<Exports>;
	}
	export namespace exports {
		export type Promisify<T> = $wcm.$exports.Promisify<T>;
	}
}

export namespace Types.$ {
	export const Range = new $wcm.RecordType<Types.Range>([
		['startLine', $wcm.u32],
		['startChar', $wcm.u32],
		['endLine', $wcm.u32],
		['endChar', $wcm.u32],
	]);
	export const Warning = new $wcm.RecordType<Types.Warning>([
		['message', $wcm.wstring],
		['range', Range],
	]);
	export const Measurement = new $wcm.RecordType<Types.Measurement>([
		['key', $wcm.u32],
		['range', Range],
	]);
	export const Results = new $wcm.RecordType<Types.Results>([
		['warnings', new $wcm.ListType<Types.Warning>(Warning)],
		['measurements', new $wcm.ListType<Types.Measurement>(Measurement)],
	]);
}
export namespace Types._ {
	export const id = 'vscode:pastelito/types' as const;
	export const witName = 'types' as const;
	export const types: Map<string, $wcm.AnyComponentModelType> = new Map<string, $wcm.AnyComponentModelType>([
		['Range', $.Range],
		['Warning', $.Warning],
		['Measurement', $.Measurement],
		['Results', $.Results]
	]);
	export type WasmInterface = {
	};
}
export namespace pastelito.$ {
	export const Results = Types.$.Results;
	export namespace exports {
		export const applyDefaultRules = new $wcm.FunctionType<pastelito.Exports['applyDefaultRules']>('apply-default-rules',[
			['input', $wcm.wstring],
		], Results);
	}
}
export namespace pastelito._ {
	export const id = 'vscode:pastelito/pastelito' as const;
	export const witName = 'pastelito' as const;
	export namespace imports {
		export const interfaces: Map<string, $wcm.InterfaceType> = new Map<string, $wcm.InterfaceType>([
			['Types', Types._]
		]);
		export function create(service: pastelito.Imports, context: $wcm.WasmContext): Imports {
			return $wcm.$imports.create<Imports>(_, service, context);
		}
		export function loop(service: pastelito.Imports, context: $wcm.WasmContext): pastelito.Imports {
			return $wcm.$imports.loop<pastelito.Imports>(_, service, context);
		}
	}
	export type Imports = {
	};
	export namespace exports {
		export const functions: Map<string, $wcm.FunctionType> = new Map([
			['applyDefaultRules', $.exports.applyDefaultRules]
		]);
		export function bind(exports: Exports, context: $wcm.WasmContext): pastelito.Exports {
			return $wcm.$exports.bind<pastelito.Exports>(_, exports, context);
		}
	}
	export type Exports = {
		'apply-default-rules': (input_ptr: i32, input_len: i32, result: ptr<Results>) => void;
	};
	export function bind(service: pastelito.Imports, code: $wcm.Code, context?: $wcm.ComponentModelContext): Promise<pastelito.Exports>;
	export function bind(service: pastelito.Imports.Promisified, code: $wcm.Code, port: $wcm.RAL.ConnectionPort, context?: $wcm.ComponentModelContext): Promise<pastelito.Exports.Promisified>;
	export function bind(service: pastelito.Imports | pastelito.Imports.Promisified, code: $wcm.Code, portOrContext?: $wcm.RAL.ConnectionPort | $wcm.ComponentModelContext, context?: $wcm.ComponentModelContext | undefined): Promise<pastelito.Exports> | Promise<pastelito.Exports.Promisified> {
		return $wcm.$main.bind(_, service, code, portOrContext, context);
	}
}