# 🚀 Sistema de Venta en Solana con Anchor Framework

![Solana](https://img.shields.io/badge/Solana-Blockchain-purple?style=for-the-badge)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blue?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-Smart%20Contracts-orange?style=for-the-badge)
![TypeScript](https://img.shields.io/badge/TypeScript-Client-blue?style=for-the-badge)

---

## 📌 Descripción del Proyecto

Este proyecto consiste en un **Sistema de Gestión de Ventas Descentralizado**, desarrollado sobre la blockchain de Solana utilizando **Anchor Framework**.

El sistema permite administrar ventas y productos de forma segura, rápida y descentralizada mediante Smart Contracts escritos en Rust.

Cada venta está vinculada a un propietario (**wallet owner**) y permite registrar múltiples productos con su respectiva información comercial.

---

## 🎯 Objetivo

Desarrollar una aplicación blockchain capaz de gestionar ventas y productos utilizando cuentas PDA (Program Derived Addresses), validaciones de propietario y lógica descentralizada sobre Solana.

---

## ⚙️ Funcionalidades Principales

### ✅ Crear Venta

Permite crear una nueva venta asociada al propietario de la wallet.

Incluye:

- Owner
- Nombre del vendedor
- Lista de productos

---

### ✅ Agregar Producto

Permite registrar nuevos productos dentro de una venta.

Cada producto contiene:

- Nombre del producto
- Cliente
- Cantidad
- Precio unitario
- Total
- Fecha de venta
- Estado de venta (activo/inactivo)

---

### ✅ Ver Productos

Muestra todos los productos registrados dentro de una venta.

---

### ✅ Eliminar Producto

Permite eliminar un producto específico mediante su nombre.

---

### ✅ Alterar Estado

Permite cambiar el estado del producto:

```text
true → activo
false → inactivo
