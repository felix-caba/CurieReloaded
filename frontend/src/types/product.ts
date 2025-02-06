import { z } from "zod";


const ProductSchema = z.object({
    idProducto: z.number().optional(),
    idLocalizacion: z.number().optional(),
    idUbicacion: z.number().optional(),
    nombre: z.string().optional(),
    cantidad: z.number().nullable().optional(),
})

const ReactivoSchema = ProductSchema.extend({
    formato: z.string().nullable().optional(),
    gradoPureza: z.string().nullable().optional(),
    fechaCaducidad: z.string().nullable().optional(),
})

const MaterialSchema = ProductSchema.extend({
    subcategoria: z.string().nullable().optional(),
    numero_serie: z.string().nullable().optional(),
    descripcion: z.string().nullable().optional(),
    fecha_compra: z.string().nullable().optional(),
    fechaCaducidad: z.string().nullable().optional(),
})

const AuxiliarSchema = ProductSchema.extend({
    formato: z.string().nullable().optional(),
})

export type Reactivo = z.infer<typeof ReactivoSchema>
export type Material = z.infer<typeof MaterialSchema>
export type Auxiliar = z.infer<typeof AuxiliarSchema>


