import { z } from "zod";


/*
export type Producto = {
    idProducto:     number;
    idLocalizacion: number;
    idUbicacion:    number;
    nombre:         string;
    cantidad:       number | null;
    stock_minimo:   number | null;
}

export type Reactivo = Producto & {
    formato:        string | null;
    gradoPureza:    string | null;
    fechaCaducidad: Date | null;
}

export type Material = Producto & {
    subcategoria: string | null;
    numero_serie: string | null;
    descripcion: string | null;
    fecha_compra: Date | null;
    fechaCaducidad: Date | null;
}

export type Auxiliar = Producto & {
    formato: string | null;
}
*/
const Producto = z.object({
    idProducto: z.number().optional(),
    idLocalizacion: z.number().optional(),
    idUbicacion: z.number().optional(),
    nombre: z.string().optional(),
    cantidad: z.number().nullable().optional(),
})

const Reactivo = Producto.extend({
    formato: z.string().nullable().optional(),
    gradoPureza: z.string().nullable().optional(),
    fechaCaducidad: z.string().nullable().optional(),
})

const Material = Producto.extend({
    subcategoria: z.string().nullable().optional(),
    numero_serie: z.string().nullable().optional(),
    descripcion: z.string().nullable().optional(),
    fecha_compra: z.string().nullable().optional(),
    fechaCaducidad: z.string().nullable().optional(),
})

const Auxiliar = Producto.extend({
    formato: z.string().nullable().optional(),
})

export type Reactivo = z.infer<typeof Reactivo>
export type Material = z.infer<typeof Material>
export type Auxiliar = z.infer<typeof Auxiliar>


